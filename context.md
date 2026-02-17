# 🗺️ Project Context: sysaudit

> **AI Instructions:** This file is the Source of Truth. Update this file during the **Phase 4: Summarize** stage of the TARS workflow.

---

## 🏗️ System Overview
* **Goal:** A high-performance Rust library and CLI for auditing Windows systems (Software, Hardware, Updates).
* **Core Stack:** Rust (Workspaces), `windows-rs`, `WMI`, `Registry`.
* **Architecture Pattern:** Modular library crate (`sysaudit`) with a CLI consumer (`sysaudit-cli`).

---

## 💻 Environment & Constraints
* **Host OS:** Windows (Non-Admin)
* **Shell Environment:** BusyBox (via Scoop) / PowerShell
* **Toolchain:** MSVC (Portable), Cargo, Rustup via Scoop
* **Deployment:** Standalone User-space binary
* **Strict Rules:**
    1. No `sudo`/Admin commands.
    2. Scripts use `#!/bin/sh` (BusyBox compatible).
    3. GUI must remain responsive during high-concurrency network scans.

---

## 📍 Current State (Recursive Summary)

### 🛠️ Recent Changes (Last 3 Cycles)
1.  **2026-02-17/Architecture:** Created `architecture.md` as the Technical Source of Truth, concretizing rules from `GEMINI.md` and data from `DATA_SOURCES.md`.
2.  **2026-02-17/Project Sync:** Committed and pushed `architecture.md`, `.gitignore`, and `Makefile` fixes to remote.
3.  **2026-02-17/BDD Specs:** Implemented BDD testing using `rstest-bdd 0.5.0` in a new `specs` workspace crate. Created 7 features and 100+ step definitions. Integrated `cargo-bdd` for diagnostics.

### 🧩 Active Components & APIs
* `sysaudit/`: Core library (Published v0.1.0).
* `sysaudit-cli/`: CLI consumer (Local-only).
* `specs/`: BDD integration tests (Requires `rstest-bdd 0.5.0`).

### 🛠️ Maintenance & Scripts
* `Makefile`: Central entry point for `check`, `run`, `test`, `build`, and `verify`.
* `cargo-bdd`: Diagnostic tool for `specs` crate (run via `cargo-bdd steps` or `cargo-bdd duplicates`).
* `scripts/test_all.sh`: Comprehensive quality gate (Lint + Test + Build).

---

### 💻 Shell & Tooling Quirks
* **rstest-bdd Uniqueness:** Step strings must be globally unique across all `*_steps.rs` files in the `specs` crate. Duplicate strings cause runtime panics due to `LazyLock` poisoning.

---

## 📜 Decision Log (The "Why")
* **2026-02-17:** Adopted `rstest-bdd 0.5.0` over `0.1.x` as the latter does not exist on crates.io.
* **2026-02-17:** Opted for independent step functions over a central `World` struct to minimize boilerplate and leverage `rstest` fixture injection.

---

## 🚧 Technical Debt & Pending Logic
* **Next Steps:** Expand hardware metric coverage and finalize CSV/JSON export alignment across all data types.

---

## 🏗️ Architecture & Documentation
* **GEMINI.md:** Operational Source of Truth (Rules & Workflows).
* **architecture.md:** Technical Source of Truth (Project-specific concretization).
* **context.md:** Contextual Source of Truth (History & Decisions).
* **DATA_SOURCES.md:** Reference for Windows Registry/WMI/API sources.

---

## 🧪 Verification Commands
```bash
# Full Quality Gate (lint + test + format check)
make check

# Manual Lint Check (Sequential)
cargo fmt -- --check
cargo clippy -- -D warnings
```