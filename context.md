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
1.  **2026-02-17/Test Audit:** Expanded unit tests from 10 to 36. Refactored `industrial.rs` and `software.rs` to extract pure logic from registry-dependent code ("Extract & Test" pattern). Fixed operator precedence bug in industrial matching logic.
2.  **2026-02-17/Architecture:** Created `architecture.md` as the Technical Source of Truth, concretizing rules from `GEMINI.md` and data from `DATA_SOURCES.md`.
3.  **2026-02-17/Project Sync:** Committed and pushed `architecture.md`, `.gitignore` (ignoring `GEMINI.md`), and `Makefile` fixes to remote.

### 🧩 Active Components & APIs
* `sysaudit/`: Core library (Published v0.1.0).
* `sysaudit-cli/`: CLI consumer (Local-only).

### 🛠️ Maintenance & Scripts
* `Makefile`: Central entry point for `check`, `run`, `test`, `build`, and `verify`.
* `scripts/test_all.sh`: Comprehensive quality gate (Lint + Test + Build).
* `BLUEPRINT_TEMPLATE.md`: Standardized format for Architect's **Think Phase** audits (includes "Files to be modified" scope).

---

### 💻 Shell & Tooling Quirks
* **PowerShell `&&` Limitation:** The default shell on this host (PowerShell) does not support `&&` as a statement separator.
    * **Solution:** Run commands sequentially in separate tool calls. **NEVER** use `&&` in `run_command` tools on Windows hosts; use `;` or separate tool calls instead.

---

## 📜 Decision Log (The "Why")
* **2026-02-11:** Substituted `rust-mcp-server` for local cargo commands to leverage unified MCP interface for build/quality cycles.
* **2026-02-11:** workspace-level formatting requires `--all` flag in `cargo fmt` to target all members.

---

## 🚧 Technical Debt & Pending Logic
* **Next Steps:** Integrate full WMI system audits and expand industrial software vendor lists.

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