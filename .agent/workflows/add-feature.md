---
description: How to add or edit a BDD feature file and its step definitions
---

# Add/Edit a BDD Feature

## Adding a New Feature

### 1. Write the `.feature` file
Create `specs/features/<name>.feature` with Gherkin syntax.

### 2. Create step definitions
Create `specs/tests/steps/<name>_steps.rs` with `#[given/when/then("...")]` functions.
Import macros: `use rstest_bdd_macros::{given, when, then};`

> **CRITICAL**: Every step string must be globally unique across ALL `*_steps.rs` files.

### 3. Register the module
Add `pub mod <name>_steps;` to `specs/tests/steps/mod.rs`.

### 4. Wire the scenario
Add to `specs/tests/bdd_runner.rs`:
```rust
#[scenario(path = "features/<name>.feature")]
fn <name>_feature() {}
```

// turbo
### 5. Run tests
```
cargo test -p specs
```

// turbo
### 6. Check for duplicates
```
cargo-bdd duplicates
```

## Editing an Existing Feature

### If changing step text:
1. Edit the `.feature` file
2. Update the matching `#[given/when/then("...")]` attribute string in `*_steps.rs`
3. Ensure no other feature depends on the old step text

### If adding a new scenario with NEW steps:
1. Add scenario to the `.feature` file
2. Add new step functions to the appropriate `*_steps.rs`

// turbo
3. Run `cargo test -p specs` — it tells you exactly which steps are missing

## Diagnostic Commands

// turbo
- `cargo-bdd steps` — List all registered steps with source locations
// turbo
- `cargo-bdd unused` — Find steps never executed (prune dead code)
// turbo
- `cargo-bdd duplicates` — Find duplicate keyword+pattern pairs
// turbo
- `cargo-bdd skipped` — List skipped scenarios and reasons
