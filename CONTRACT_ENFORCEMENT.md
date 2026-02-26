# API Contract Enforcement Strategy

How `canonical-errors` prevents accidental changes in error processing from breaking the API contract.

## Enforcement Tiers

### Tier 1: Compile-Time (Rust Type System)

**Already implemented.** The strongest tier — broken code cannot compile.

| Mechanism                                                                | What it prevents                                             |
|--------------------------------------------------------------------------|--------------------------------------------------------------|
| Typed enum variants (`NotFound` carries `ResourceInfo`, not `ErrorInfo`) | Using the wrong context type for a category                  |
| Exhaustive `match` in `From<CanonicalError> for Problem`                 | Forgetting to handle a new error category                    |
| `resource_error!` macro                                                  | Forgetting to set `resource_type` for resource-scoped errors |
| `GtsSchema` trait with `SCHEMA_ID` constant                              | Schema ID typos caught at compile time via associated const  |

**Potential additions:**

- `#[non_exhaustive]` on `CanonicalError` — allows adding variants without a semver-major bump, forcing downstream consumers to handle unknown variants
- Sealed `ErrorContext` trait — prevents external crates from creating custom context types that might serialize incorrectly

### Tier 2: Test-Time (Serialization Verification)

**Partially implemented.** Catches changes to the actual JSON wire format.

#### Currently in place

- **16 showcase tests** — full `assert_eq!` against inline `serde_json::json!({...})` for every error category's Problem JSON output
- **12 schema tests** — full JSON equality assertions for every context type's GTS-generated JSON Schema

#### Potential additions

**Snapshot testing with [`insta`](https://insta.rs/)**

- Replace inline `assert_eq!` with `assert_json_snapshot!` managed by `.snap` files
- Snapshot files are version-controlled — changes show as clear diffs in every PR
- `cargo insta review` forces explicit human approval of any contract change
- `cargo insta test --check` in CI fails if unapproved changes exist

**Schema cross-validation with [`jsonschema`](https://docs.rs/jsonschema)**

- Serialize each error variant → validate the output against the GTS-generated JSON Schema
- Catches drift between serde serialization code and schema definitions
- Example: if someone adds `#[serde(rename = "foo")]` but the schema still says `"bar"`, cross-validation fails

### Tier 3: CI-Time (Pre-Merge Gates)

**Not yet implemented.** Catches issues before code reaches the main branch.

**`cargo-semver-checks`**

- Analyzes the crate's public API via rustdoc JSON
- Detects: removed types, changed function signatures, removed enum variants, changed trait bounds
- GitHub Action: `obi1kenobi/cargo-semver-checks-action@v2`
- Limitation: does not catch serialization-level changes (field names, JSON structure) — that's what Tier 2 covers

**Schema file diffing**

- Export all GTS schemas to `schemas/*.json` files and commit them
- CI regenerates schemas, diffs against committed versions, fails if different
- Makes every schema evolution visible as a PR diff — reviewers can assess whether the change is intentional

**Snapshot CI gate**

- `cargo insta test --check` in CI rejects any test run where snapshots don't match
- Developers must run `cargo insta review` locally and commit approved `.snap` files

### Tier 4: Design-Time (Architecture)

**Implemented through conventions.** The crate's architecture inherently discourages contract violations.

| Pattern                                                     | How it helps                                                             |
|-------------------------------------------------------------|--------------------------------------------------------------------------|
| Single `Problem` conversion point (`From<CanonicalError>`)  | All 16 categories go through one code path — no ad-hoc JSON construction |
| Context types are value objects with dedicated constructors | e.g. `ResourceInfo::new(type, name)` — can't forget required fields      |
| `GtsSchema` generates schemas from the types themselves     | Schema and code can't disagree on field names (same source of truth)     |
| Default messages per category                               | Standard messages like "Resource not found" prevent inconsistent wording |

## Coverage Matrix

| What could go wrong               | Tier 1 (Compile) | Tier 2 (Test)              | Tier 3 (CI)                   |
|-----------------------------------|------------------|----------------------------|-------------------------------|
| Wrong context type for a category | Caught           | —                          | —                             |
| Missing match arm for new variant | Caught           | —                          | —                             |
| Field renamed in serialization    | —                | Caught (snapshot)          | Caught (schema diff)          |
| Default message changed           | —                | Caught (snapshot)          | —                             |
| Status code changed               | —                | Caught (snapshot)          | —                             |
| New field added to context type   | —                | Caught (snapshot + schema) | Caught (schema diff)          |
| Field removed from context type   | Caught (if used) | Caught (snapshot + schema) | Caught (schema diff + semver) |
| Schema/serialization drift        | —                | Caught (cross-validation)  | —                             |
| Public type removed or renamed    | Caught           | —                          | Caught (semver)               |
| Function signature changed        | Caught           | —                          | Caught (semver)               |

## Implementation Priorities

1. **Tier 1 and Tier 4 are already in place** — no action needed
2. **Tier 2 snapshot testing** — highest value, lowest effort. Add `insta` as dev-dependency, migrate existing tests
3. **Tier 2 cross-validation** — moderate effort. Add `jsonschema` as dev-dependency, write one test
4. **Tier 3 CI gates** — add when publishing the crate or when the team grows. Requires GitHub Actions setup
