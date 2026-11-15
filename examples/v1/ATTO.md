# AttoLang projection (candidate)

`atto-clip-duration.atto` models the bounded `clip` output duration policy
from `schemas/v1/creator-media-project.schema.json`: 30,000 through
50,000 milliseconds inclusive. The JSON Schema is the authoritative source.

`atto-contract-candidate.json` explicitly blocks production promotion.
`cargo test --locked --test atto_projection` checks for schema/source drift,
boundary regressions, and unjustified admission using negative canaries.
The test is a **restricted-source alignment test**, not proof that Atto
currently compiles or runs the example.

Before production use, require a pinned Atto Rust/Wasm compiler to parse,
typecheck, emit, and execute the fixture. Then verify outputs at the four
boundaries against the authoritative schema and document a reproducible
compiler SHA/receipt. Do not distribute as a validator or renderer until that
work is complete.
