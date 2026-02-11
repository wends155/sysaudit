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
    2. Scripts must be `#!/bin/sh` (BusyBox compatible).
    3. GUI must remain responsive during high-concurrency network scans.

---

## 📍 Current State (Recursive Summary)

### 🛠️ Recent Changes (Last 3 Cycles)
1.  **2026-02-11/Documentation:** Updated `context.md` to match `sysaudit` project reality.
2.  **2026-02-11/MCP Integration:** Successfully verified `rust-mcp-server` tools (`cargo-build`, `cargo-clippy`, `cargo-fmt`) against the workspace.

### 🧩 Active Components & APIs
* `sysaudit/`: Core logic (system info, software/industrial scans, updates).
* `sysaudit-cli/`: Command-line interface for the auditor.

### 🛠️ Maintenance & Scripts
* `Makefile`: Central entry point for `check`, `run`, `test`, `build`, and `verify`.
* `scripts/verify.sh`: Comprehensive quality gate (Lint + Test + Build).
* `BLUEPRINT_TEMPLATE.md`: Standardized format for Architect's **Think Phase** audits (includes "Files to be modified" scope).

---

### 💻 Shell & Tooling Quirks
* **PowerShell `&&` Limitation:** The default shell on this host (PowerShell) does not support `&&` as a statement separator.
    * **Solution:** Run commands sequentially in separate tool calls. Do not use `&&` in `run_command` tools.

---

## 📜 Decision Log (The "Why")
* **2026-02-11:** Substituted `rust-mcp-server` for local cargo commands to leverage unified MCP interface for build/quality cycles.
* **2026-02-11:** workspace-level formatting requires `--all` flag in `cargo fmt` to target all members.

---

## 🚧 Technical Debt & Pending Logic
* **Next Steps:** Integrate full WMI system audits and expand industrial software vendor lists.

---

## 🧪 Verification Commands
```bash
# Full Quality Gate
make verify

# Manual Lint Check (Sequential)
cargo fmt -- --check
cargo clippy -- -D warnings
```