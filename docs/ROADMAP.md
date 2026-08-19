# Forge Roadmap

This roadmap is the project's durable engineering memory. Every architectural loop must update it before moving on. Quality and evidence determine progression.

## Product contract

- Native Rust application.
- No runtime dependency on AI, OpenAI, ChatGPT, Ollama, model servers, or external AI services.
- Offline operation for capabilities that do not inherently require a network.
- Cross-platform ambition is Windows + Linux + macOS, subject to real testing.
- Forge v0.1 is a shell foundation, not a terminal emulator and not a full Bash/PowerShell replacement.
- Small, explicit abstractions are preferred over speculative frameworks.

## Engineering policy

- **Reuse Before Reinvent:** research mature libraries and implementations before building substantial infrastructure.
- Reuse only after reviewing license compatibility, maintenance health, security, portability, performance, dependency cost, and replaceability.
- Never copy third-party source blindly. Preserve required notices and licenses; implement independently when only concepts or public interfaces are being studied.
- Record important reuse research and decisions so future loops do not repeat the same investigation.

## Foundation — v0.1

- [x] Native Rust application
- [x] Interactive line editing
- [x] Persistent command history with opt-out
- [x] Built-in help
- [x] Built-in version
- [x] Working-directory navigation
- [x] Native process execution
- [x] Structured argument parsing for words, quotes, and escapes
- [x] Actionable command errors
- [x] Unit tests for parser edge cases
- [x] Cross-platform CI quality gate definition
- [x] Architecture documentation
- [x] Modular shell/parser/process/history boundaries
- [x] Initial process-execution error tests
- [x] Reuse Before Reinvent policy
- [x] Centralized platform-aware user-state path boundary
- [x] Configuration architecture proposal
- [x] Declarative TOML configuration implementation
- [x] Separate configuration and persistent-state paths
- [x] Unknown configuration settings rejected explicitly
- [x] Configuration contract documented
- [x] Deterministic testing strategy documented

## Current LOOP — reliability before language expansion

The project is **not yet declared usable**. The current loop is focused on proving the foundation rather than adding visible features.

### In progress

- [ ] Obtain and verify a real CI run on the current main branch.
- [ ] Commit and verify a real Cargo-resolved dependency lockfile.
- [x] Add deterministic unit coverage for successful and nonzero native process exit statuses on supported operating systems.
- [ ] Deterministic process-execution integration tests.
- [ ] Explicit exit-status model and semantics.
- [ ] Security review of environment and process inheritance.
- [ ] UX review of prompt, diagnostics, interruption, and EOF behavior.
- [ ] Configuration integration tests, including malformed and unknown settings.
- [ ] Clean-machine installation smoke test.
- [ ] Release and installation strategy for a clean machine.
- [ ] Red Team review of the complete current codebase.
- [ ] Research mature shell projects and crates before expanding the shell language.

### Current observations

- The parser intentionally supports only words, quotes, and escapes. Operators are blocked until a grammar and semantic model are designed.
- Native process execution is deliberately isolated from future shell-language features.
- Persistent history is opt-out and uses a centralized platform-aware user-state boundary; Unix history files are private 0600 files.
- Configuration is declarative TOML and currently supports only `[ui].prompt`; it never executes commands.
- Configuration defaults are overridden by the user configuration file. CLI/environment overrides are not yet implemented and must not be implied by documentation.
- Unknown configuration settings are rejected rather than silently ignored.
- A deterministic testing strategy is documented in `docs/TESTING.md`.
- The current GitHub integration can write repository files, but a verified CI execution has not yet been observed. Therefore CI is not considered validated.
- `Cargo.lock` is currently only a generated header with no resolved packages. It must not be described as a verified reproducible dependency lock until a real Cargo resolution/build has populated and validated it.
- The GitHub contents API rejected a process.rs update because the remote blob SHA did not match the write precondition. The file was re-read instead of force-overwriting it. A related test change was then applied safely to `src/lib.rs` using its current SHA.

### Decisions made in this loop

- Reuse research continues to favor mature shell projects as behavioral references rather than copying implementation wholesale.
- Native process tests should use deterministic commands supplied by the target OS (`true`/`false` on Unix and `cmd /C exit N` on Windows) rather than optional developer-installed programs.
- Successful and nonzero process exit statuses now have explicit unit coverage on both supported OS families.
- Do not claim those tests passed until Cargo or CI actually executes them.
- A failed optimistic write must never be retried with a stale SHA; re-read the remote file and apply the smallest safe change.

### Reuse research already identified

- Brush: reference for parser/runtime separation, interactive behavior, testing, configuration, and platform concerns.
- ReShell: reference for parser/checker/runtime/builtins/REPL separation.
- DataDog rshell: reference for security and capability-oriented execution ideas, with different product goals.
- Conch: reference for project organization, packaging, and cross-platform engineering.
- `directories`: candidate for future platform path handling; not adopted yet.
- Nushell and fish: references for mature configuration and startup ordering; Forge deliberately avoids executable startup configuration for v0.1.
- `toml` + `serde`: selected for the current declarative configuration implementation after reuse review at the design level.

These projects are references, not automatic dependencies. Each future adoption decision requires its own license, maintenance, security, portability, performance, and architectural review.

## Shell language — architectural approval required

These features remain blocked until grammar, semantics, security model, UX, portability, and tests have been reviewed by the full LOOP.

- [ ] operator grammar
- [ ] pipes
- [ ] input/output redirection
- [ ] command chaining
- [ ] environment expansion
- [ ] globbing
- [ ] background jobs and job control

## Later — only after architectural approval

- [ ] completion
- [ ] plugins
- [ ] scripting
- [ ] Git-aware tooling
- [ ] terminal emulator
- [ ] richer configuration UX
- [ ] distribution/package channels

No later item is a promise. Every item must earn its place through user value, technical feasibility, security review, platform testing, maintainability, and measurable UX quality.

## Definition of Done — Forge v0.1

Forge v0.1 may be called **usable** only when all of the following are true:

- [ ] clean build succeeds on every supported CI platform
- [ ] formatting and Clippy gates pass without warnings
- [ ] unit and integration tests pass
- [ ] core shell workflows work in a real terminal
- [ ] process failures are understandable and recoverable
- [ ] filesystem navigation is reliable across supported platforms
- [ ] history behavior is documented and safe
- [ ] configuration behavior is documented and tested
- [ ] installation works on a clean supported machine
- [ ] release artifact can be identified and reproduced
- [ ] security review has no unresolved high-severity findings
- [ ] Red Team has attempted to break the MVP
- [ ] UX review has no known critical usability failures
- [ ] documentation matches shipped behavior
- [ ] architecture review finds no known severe foundational flaw

Until these gates are met, the status remains **IN DEVELOPMENT**.

## LOOP protocol

For every meaningful change:

`ANALYZE → RESEARCH → PROPOSE → CRITICIZE → SIMPLIFY → IMPLEMENT → TEST → RED TEAM → CORRECT → REVIEW → DOCUMENT → UPDATE ROADMAP → COMMIT → REPEAT`

If a new uncertainty could cause major structural rework, stop and resolve it before implementation. Never mark a feature complete solely because it compiles.
