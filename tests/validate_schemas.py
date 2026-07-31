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

secret_patterns = [
    re.compile(r"ghp_[A-Za-z0-9]{20,}"),
    re.compile(r"AIza[0-9A-Za-z_-]{20,}"),
    re.compile(r"-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----"),
]
for path in ROOT.rglob("*"):
    if not path.is_file() or ".git" in path.parts:
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
print("- no obvious secrets or conflict markers found")
