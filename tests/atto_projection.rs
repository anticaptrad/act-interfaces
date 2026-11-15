//! Candidate-only AttoLang contract projection tests.
//! These tests check schema/source/manifest alignment, NOT compiler execution.
use serde_json::{json, Value};

const SOURCE: &str = include_str!("../examples/v1/atto-clip-duration.atto");
const CANDIDATE: &str = include_str!("../examples/v1/atto-contract-candidate.json");
const SCHEMA: &str = include_str!("../schemas/v1/creator-media-project.schema.json");

fn errors(candidate: &Value, schema: &Value, source: &str) -> Vec<&'static str> {
    let mut failures = Vec::new();
    let mut check = |valid: bool, reason| {
        if !valid {
            failures.push(reason);
        }
    };

    check(candidate["schemaVersion"] == 1, "version mismatch");
    check(candidate["language"] == "atto", "wrong language");
    check(candidate["status"] == "candidate", "unapproved promotion");
    check(
        candidate["compilerEvidence"] == "unverified",
        "compiler proof missing",
    );
    check(
        candidate["runtimeRenderer"] == false,
        "unsupported renderer claim",
    );
    check(
        candidate["source"] == "examples/v1/atto-clip-duration.atto",
        "untrusted source path",
    );
    check(
        candidate["contractAuthority"] == "schemas/v1/creator-media-project.schema.json",
        "authority path changed",
    );
    check(
        candidate["projection"] == "output.kind=clip -> output.durationMs",
        "projection changed",
    );

    let clip_rule = &schema["$defs"]["output"]["allOf"][0];
    check(
        clip_rule["if"]["properties"]["kind"]["const"] == "clip",
        "wrong guard kind",
    );
    check(
        clip_rule["then"]["required"]
            .as_array()
            .is_some_and(|a| a.iter().any(|v| v == "sourceWindow")),
        "clip source window requirement missing",
    );
    let bounds = &clip_rule["then"]["properties"]["durationMs"];
    let min = bounds["minimum"].as_i64();
    let max = bounds["maximum"].as_i64();
    check(min.is_some() && max.is_some(), "noninteger schema bounds");

    if let (Some(min), Some(max)) = (min, max) {
        check(min <= max && min >= 0, "invalid schema range");
        check(
            candidate["bounds"]["minInclusive"] == min,
            "minimum mismatch",
        );
        check(
            candidate["bounds"]["maxInclusive"] == max,
            "maximum mismatch",
        );

        // Keep the candidate intentionally restricted to a reviewable, known
        // syntax shape. Fail closed on any source/control-flow mutation.
        let normalized = source
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        let expected = format!(
            "fnc clip_duration_valid(int duration_ms): bool {{\nif duration_ms < {min} then\nreturn false;\nfi\nreturn duration_ms <= {max};\n}}\n"
        );
        check(normalized.starts_with(&expected), "Atto source guard drift");
        check(
            normalized.contains("pub fnc main(): int {"),
            "missing entrypoint for future compiler smoke",
        );
        for edge in [min - 1, min, max, max + 1] {
            let present = normalized.contains(&format!("clip_duration_valid({edge})"));
            check(present, "missing boundary fixture");
        }
    }
    check(
        !source.contains("Authorization:")
            && !source.contains("bearerToken")
            && !source.contains("upload_video("),
        "unauthorized transport or secret surface",
    );

    failures
}

#[test]
fn candidate_projection_aligns_with_authoritative_clip_schema() {
    let candidate: Value = serde_json::from_str(CANDIDATE).unwrap();
    let schema: Value = serde_json::from_str(SCHEMA).unwrap();
    assert_eq!(errors(&candidate, &schema, SOURCE), Vec::<&str>::new());
}

#[test]
fn mutated_schema_projection_and_unsupported_promotion_fail_closed() {
    let candidate: Value = serde_json::from_str(CANDIDATE).unwrap();
    let schema: Value = serde_json::from_str(SCHEMA).unwrap();
    let mut bad_schema = schema.clone();
    bad_schema["$defs"]["output"]["allOf"][0]["then"]["properties"]["durationMs"]["maximum"] =
        json!(50001);
    assert!(!errors(&candidate, &bad_schema, SOURCE).is_empty());

    let mut bad_candidate = candidate.clone();
    bad_candidate["status"] = json!("production");
    assert!(!errors(&bad_candidate, &schema, SOURCE).is_empty());

    let mut false_admission = candidate.clone();
    false_admission["compilerEvidence"] = json!("passed");
    assert!(!errors(&false_admission, &schema, SOURCE).is_empty());

    let bad_source = SOURCE.replace("return false;", "return true;");
    assert!(!errors(&candidate, &schema, &bad_source).is_empty());
}
