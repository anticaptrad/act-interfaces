# act-interfaces

Versioned, language-neutral contracts shared by Anticaptrad services.

## Lifecycle

**Profile:** contract repository  
**Status:** active baseline; not a deployed service  
**Owner:** Anticaptrad platform maintainers

This repository is the canonical place for stable request, response, and event schemas at the Rust control-plane boundary. Runtime implementations live in repositories such as `act-api-server.rs`; generated clients belong in `act-clients`.

## Current contract set

`schemas/v1/` describes:

- a transport-neutral YouTube control request;
- the success/error envelope returned by the Rust control plane;
- redacted NATS lifecycle events on `act.youtube.<action>.<phase>`.

The HTTP mapping implemented by `act-api-server.rs` is:

- `action` → `/v1/youtube/actions/{action}` path segment;
- `idempotencyKey` → `Idempotency-Key` header for mutating actions;
- `payload` → JSON request body;
- administrative authentication → `Authorization: Bearer …` outside these schemas.

Secrets, OAuth tokens, API keys, deployment IDs used as credentials, and raw upstream error bodies are intentionally excluded.

## Validation

```bash
python3 tests/validate_schemas.py
```

The validator uses only the Python standard library. CI checks JSON syntax, stable schema identifiers, exact action coverage, mutating-action idempotency requirements, response-envelope exclusivity, and event-topic compatibility.

## Compatibility policy

- Existing required fields and enum values are not removed in-place.
- Breaking changes require a new version directory.
- New optional fields must remain fail-closed at runtime.
- Publishing remains private by default; public/unlisted transitions are not inferred by schemas.

## Related repositories

- `anticaptrad/act-api-server.rs` — guarded runtime implementation.
- `anticaptrad/act-clients` — generated and handwritten clients.
- `anticaptrad/act-sync` — reconciliation model consuming these lifecycle contracts.

Licensed under the MIT License.
