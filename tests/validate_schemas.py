#!/usr/bin/env python3
"""Deterministic contract validation using only the Python standard library."""

from __future__ import annotations

import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
SCHEMAS = ROOT / "schemas" / "v1"
EXPECTED_ACTIONS = [
    "channel",
    "videos",
    "analytics",
    "exportAnalytics",
    "jobs",
    "startUpload",
    "processUpload",
    "processAllUploads",
    "publishVideo",
    "updateVideo",
    "createPlaylist",
    "addToPlaylist",
    "ingestGmail",
    "sendDigest",
    "partnerStatus",
    "partnerOwners",
    "partnerClaims",
    "adminStatus",
    "workspaceUsers",
]
EXPECTED_MUTATIONS = [
    "exportAnalytics",
    "startUpload",
    "processUpload",
    "processAllUploads",
    "publishVideo",
    "updateVideo",
    "createPlaylist",
    "addToPlaylist",
    "ingestGmail",
    "sendDigest",
]
IGNORED_SCAN_DIRECTORIES = {
    ".dart_tool",
    ".git",
    ".gradle",
    ".ores",
    ".venv",
    "build",
    "dist",
    "node_modules",
    "target",
    "zed_modules",
}

errors: list[str] = []


def check(condition: bool, message: str) -> None:
    if not condition:
        errors.append(message)


def load(name: str) -> dict:
    path = SCHEMAS / name
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except Exception as exc:  # pragma: no cover - diagnostic path
        errors.append(f"{path.relative_to(ROOT)} is invalid JSON: {exc}")
        return {}
    check(value.get("$schema") == "https://json-schema.org/draft/2020-12/schema", f"{name}: wrong JSON Schema draft")
    check(value.get("$id") == f"https://schemas.anticaptrad.org/v1/{name}", f"{name}: unstable $id")
    return value


request = load("youtube-control-request.schema.json")
response = load("youtube-control-response.schema.json")
event = load("youtube-lifecycle-event.schema.json")
creator_project = load("creator-media-project.schema.json")
render_receipt = load("creator-render-receipt.schema.json")

request_actions = request.get("properties", {}).get("action", {}).get("enum", [])
event_actions = event.get("properties", {}).get("action", {}).get("enum", [])
check(request_actions == EXPECTED_ACTIONS, "request action enum does not match the Rust control plane")
check(event_actions == EXPECTED_ACTIONS, "event action enum must exactly match request actions")

mutation_rule = request.get("allOf", [{}])[0]
mutations = mutation_rule.get("if", {}).get("properties", {}).get("action", {}).get("enum", [])
required_for_mutation = mutation_rule.get("then", {}).get("required", [])
check(mutations == EXPECTED_MUTATIONS, "mutating action set is incomplete or reordered")
check(required_for_mutation == ["idempotencyKey"], "mutations must require idempotencyKey")

branches = response.get("oneOf", [])
check(len(branches) == 2, "response must have exactly one success and one failure branch")
if len(branches) == 2:
    constants = [branch.get("properties", {}).get("ok", {}).get("const") for branch in branches]
    check(constants == [True, False], "response branches must be success then failure")
    check("data" in branches[0].get("required", []), "success response must require data")
    check("error" in branches[1].get("required", []), "failure response must require error")

phases = event.get("properties", {}).get("phase", {}).get("enum", [])
check(phases == ["requested", "succeeded", "failed"], "lifecycle event phases are unstable")
check(event.get("additionalProperties") is False, "event schema must reject unknown top-level fields")

project_properties = creator_project.get("properties", {})
publication = creator_project.get("$defs", {}).get("publication", {}).get("properties", {})
check(project_properties.get("schemaVersion", {}).get("const") == "1.0", "creator project version is unstable")
check(
    project_properties.get("creatorHandle", {}).get("const") == "@anticaptrad",
    "creator project must pin @anticaptrad",
)
check(publication.get("channelId", {}).get("const") == "UC-Gloecwemo_Mh-VAjnUipg", "wrong YouTube channel pin")
check(publication.get("privacyStatus", {}).get("const") == "private", "creator project must publish privately")
check(publication.get("allowPublic", {}).get("const") is False, "creator project cannot shortcut public publishing")

project_defs = creator_project.get("$defs", {})
licensed_rights_rule = project_defs.get("rights", {}).get("allOf", [{}])[0]
licensed_required = licensed_rights_rule.get("then", {}).get("required", [])
check(
    licensed_required == ["sourceUrl", "licenseId", "licenseName", "attribution"],
    "licensed stock and cues must carry complete provenance",
)
clip_rule = project_defs.get("output", {}).get("allOf", [{}])[0]
clip_duration = clip_rule.get("then", {}).get("properties", {}).get("durationMs", {})
check(
    clip_duration.get("minimum") == 30000 and clip_duration.get("maximum") == 50000,
    "creator clips must remain between 30 and 50 seconds",
)

receipt_defs = render_receipt.get("$defs", {})
receipt_publication = receipt_defs.get("publicationReceipt", {}).get("properties", {})
check(receipt_publication.get("publicEligible", {}).get("const") is False, "render receipts cannot approve public release")
check(receipt_publication.get("privacyStatus", {}).get("const") == "private", "render receipts must remain private-first")
failure_codes = receipt_defs.get("failure", {}).get("properties", {}).get("code", {}).get("enum", [])
check("RIGHTS_NOT_APPROVED" in failure_codes, "render failure contract must expose a rights gate")
check("ASSET_DIGEST_MISMATCH" in failure_codes, "render failure contract must expose digest mismatch")

for schema_name, schema in (("creator project", creator_project), ("render receipt", render_receipt)):
    relative_path_pattern = schema.get("$defs", {}).get("relativePath", {}).get("pattern", "")
    check("(?!/)" in relative_path_pattern, f"{schema_name} paths must reject absolute paths")
    check("\\.\\." in relative_path_pattern, f"{schema_name} paths must reject traversal")
    check(schema.get("additionalProperties") is False, f"{schema_name} must reject unknown top-level fields")

secret_patterns = [
    re.compile(r"ghp_[A-Za-z0-9]{20,}"),
    re.compile(r"AIza[0-9A-Za-z_-]{20,}"),
    re.compile(r"-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----"),
]
for path in ROOT.rglob("*"):
    if not path.is_file() or any(part in IGNORED_SCAN_DIRECTORIES for part in path.parts):
        continue
    text = path.read_text(encoding="utf-8", errors="ignore")
    for pattern in secret_patterns:
        if pattern.search(text):
            errors.append(f"possible secret in {path.relative_to(ROOT)}")
    for line_number, line in enumerate(text.splitlines(), start=1):
        if line.startswith(("<<<<<<<", "=======", ">>>>>>>")):
            errors.append(f"conflict marker in {path.relative_to(ROOT)}:{line_number}")

if errors:
    print("VALIDATION FAILED")
    for error in errors:
        print(f"- {error}")
    sys.exit(1)

print("VALIDATION PASSED")
print(f"- {len(EXPECTED_ACTIONS)} actions")
print(f"- {len(EXPECTED_MUTATIONS)} mutating actions require idempotency")
print("- success/error envelopes and lifecycle phases verified")
print("- creator project, rights, clip, receipt, and private-publication gates verified")
print("- no obvious secrets or conflict markers found")
