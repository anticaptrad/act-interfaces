//! Rust replacement for the versioned contract schema and repository policy checks.
//! The existing Python validator is retained temporarily for parity audit.

use std::{fs, path::Path};
use serde_json::{json, Value};

const ACTIONS: &[&str] = &[
    "channel", "videos", "analytics", "exportAnalytics", "jobs", "startUpload",
    "processUpload", "processAllUploads", "publishVideo", "updateVideo",
    "createPlaylist", "addToPlaylist", "ingestGmail", "sendDigest",
    "partnerStatus", "partnerOwners", "partnerClaims", "adminStatus", "workspaceUsers",
];
const MUTATIONS: &[&str] = &[
    "exportAnalytics", "startUpload", "processUpload", "processAllUploads",
    "publishVideo", "updateVideo", "createPlaylist", "addToPlaylist",
    "ingestGmail", "sendDigest",
];

struct Authorities {
    request: Value,
    response: Value,
    event: Value,
    project: Value,
    receipt: Value,
}

fn read_schema(name: &str) -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("schemas/v1").join(name);
    let raw = fs::read_to_string(&path).expect("schema file required");
    serde_json::from_str(&raw).expect("schema JSON required")
}

fn authorities() -> Authorities {
    Authorities {
        request: read_schema("youtube-control-request.schema.json"),
        response: read_schema("youtube-control-response.schema.json"),
        event: read_schema("youtube-lifecycle-event.schema.json"),
        project: read_schema("creator-media-project.schema.json"),
        receipt: read_schema("creator-render-receipt.schema.json"),
    }
}

fn failures(a: &Authorities) -> Vec<&'static str> {
    let mut errors = Vec::new();
    let mut check = |ok: bool, reason| {
        if !ok { errors.push(reason); }
    };
    for (name, doc) in [
        ("youtube-control-request.schema.json", &a.request),
        ("youtube-control-response.schema.json", &a.response),
        ("youtube-lifecycle-event.schema.json", &a.event),
        ("creator-media-project.schema.json", &a.project),
        ("creator-render-receipt.schema.json", &a.receipt),
    ] {
        check(doc["$schema"] == "https://json-schema.org/draft/2020-12/schema", "JSON Schema draft");
        check(doc["$id"] == format!("https://schemas.anticaptrad.org/v1/{name}"), "stable schema id");
    }

    check(a.request["properties"]["action"]["enum"] == json!(ACTIONS), "request actions");
    check(a.event["properties"]["action"]["enum"] == json!(ACTIONS), "event action parity");
    let mutating = &a.request["allOf"][0];
    check(mutating["if"]["properties"]["action"]["enum"] == json!(MUTATIONS), "idempotency action set");
    check(mutating["then"]["required"] == json!(["idempotencyKey"]), "idempotency required");
    let response_branches = a.response["oneOf"].as_array();
    check(response_branches.is_some_and(|v| v.len() == 2), "two response branches");
    if let Some(branches) = response_branches.filter(|v| v.len() == 2) {
        check(branches[0]["properties"]["ok"]["const"] == true, "success discriminant");
        check(branches[1]["properties"]["ok"]["const"] == false, "failure discriminant");
        check(branches[0]["required"].as_array().is_some_and(|v| v.iter().any(|x| x == "data")), "success data");
        check(branches[1]["required"].as_array().is_some_and(|v| v.iter().any(|x| x == "error")), "failure error");
    }
    check(a.event["properties"]["phase"]["enum"] == json!(["requested", "succeeded", "failed"]), "event phases");
    check(a.event["additionalProperties"] == false, "closed event schema");
    check(a.project["properties"]["schemaVersion"]["const"] == "1.0", "project version");
    check(a.project["properties"]["creatorHandle"]["const"] == "@anticaptrad", "creator handle");
    let publication = &a.project["$defs"]["publication"]["properties"];
    check(publication["channelId"]["const"] == "UC-Gloecwemo_Mh-VAjnUipg", "channel pin");
    check(publication["privacyStatus"]["const"] == "private", "private publication");
    check(publication["allowPublic"]["const"] == false, "no public shortcut");

    let rights = &a.project["$defs"]["rights"]["allOf"][0];
    check(rights["then"]["required"] == json!(["sourceUrl", "licenseId", "licenseName", "attribution"]),
        "licensed rights provenance");
    let clip = &a.project["$defs"]["output"]["allOf"][0]["then"]["properties"]["durationMs"];
    check(clip["minimum"] == 30000 && clip["maximum"] == 50000, "30 to 50 second clip");
    let receipt_publication = &a.receipt["$defs"]["publicationReceipt"]["properties"];
    check(receipt_publication["publicEligible"]["const"] == false, "receipt cannot authorize public");
    check(receipt_publication["privacyStatus"]["const"] == "private", "receipt privacy");
    let codes = a.receipt["$defs"]["failure"]["properties"]["code"]["enum"].as_array();
    check(codes.is_some_and(|v| v.iter().any(|x| x == "RIGHTS_NOT_APPROVED")), "rights failure");
    check(codes.is_some_and(|v| v.iter().any(|x| x == "ASSET_DIGEST_MISMATCH")), "digest failure");

    for doc in [&a.project, &a.receipt] {
        let pattern = doc["$defs"]["relativePath"]["pattern"].as_str().unwrap_or("");
        check(pattern.contains("(?!/)"), "absolute path rejection");
        check(pattern.contains(r"\.\."), "path traversal rejection");
        check(doc["additionalProperties"] == false, "no extra top-level fields");
    }
    errors
}

fn has_obvious_secret(content: &str) -> bool {
    for prefix in ["ghp_", "AIza"] {
        let required = 20;
        for (start, _) in content.match_indices(prefix) {
            let suffix = &content[start + prefix.len()..];
            if suffix.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-').count() >= required {
                return true;
            }
        }
    }
    ["", "RSA ", "EC ", "OPENSSH "].iter().any(|kind| {
        let marker = format!("{}{}{}", "-----BEGIN ", kind, "PRIVATE KEY-----");
        content.contains(&marker)
    })
}

fn repository_errors(root: &Path) -> Vec<String> {
    const IGNORED: &[&str] = &[
        ".dart_tool", ".git", ".gradle", ".ores", ".venv", "build", "dist",
        "node_modules", "target", "zed_modules",
    ];
    fn walk(folder: &Path, failures: &mut Vec<String>) {
        let entries = fs::read_dir(folder).expect("repository directory readable");
        for entry in entries {
            let entry = entry.expect("directory entry");
            let path = entry.path();
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if IGNORED.contains(&name.as_ref()) { continue; }
            if path.is_dir() {
                walk(&path, failures);
            } else if path.is_file() {
                let bytes = fs::read(&path).expect("file readable");
                let contents = String::from_utf8_lossy(&bytes);
                if has_obvious_secret(&contents) {
                    failures.push(format!("possible secret in {}", path.display()));
                }
                for (index, line) in contents.lines().enumerate() {
                    if ["<<<<<<<", "=======", ">>>>>>>"].iter().any(|marker| line.starts_with(marker)) {
                        failures.push(format!("conflict marker {}:{}", path.display(), index + 1));
                    }
                }
            }
        }
    }
    let mut errors = Vec::new();
    walk(root, &mut errors);
    errors
}

#[test]
fn versioned_schemas_match_authoritative_policy() {
    let a = authorities();
    assert!(failures(&a).is_empty(), "{:?}", failures(&a));
}

#[test]
fn mutations_reject_missing_rights_clip_and_publication_guards() {
    let mut a = authorities();
    a.request["allOf"][0]["then"]["required"] = json!([]);
    assert!(!failures(&a).is_empty());
    let mut a = authorities();
    a.project["$defs"]["output"]["allOf"][0]["then"]["properties"]["durationMs"]["maximum"] = json!(50001);
    assert!(!failures(&a).is_empty());
    let mut a = authorities();
    a.receipt["$defs"]["publicationReceipt"]["properties"]["publicEligible"]["const"] = json!(true);
    assert!(!failures(&a).is_empty());
}

#[test]
fn no_secrets_or_merge_conflict_markers_in_tracked_sources() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let issues = repository_errors(root);
    assert!(issues.is_empty(), "{issues:?}");
    assert!(has_obvious_secret(&format!("ghp_{}", "Q".repeat(24))));
    assert!(!has_obvious_secret("ghp_short_example"));
    assert!(has_obvious_secret(&format!("{}{}{}", "-----BEGIN ", "RSA ", "PRIVATE KEY-----")));
}
