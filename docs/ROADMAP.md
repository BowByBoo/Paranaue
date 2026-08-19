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
- [x] Add shell navigation invariant tests for current-directory initialization, relative `cd`, and invalid `cd` arity.
- [x] Add shell error-path tests for missing directories, regular-file targets, and unknown commands.
- [ ] UX review of prompt, diagnostics, interruption, and EOF behavior.
- [x] Configuration integration tests for missing, valid, malformed, and unknown settings using explicit test paths.
- [ ] Clean-machine installation smoke test.
- [ ] Release and installation strategy for a clean machine.
- [ ] Red Team review of the complete current codebase.
- [ ] Research mature shell projects and crates before expanding the shell language.
- [x] Red Team audit of parser state/error boundaries completed; no shell operators or expansion semantics added.
- [ ] Parser fuzz/property testing strategy.
- [x] Ctrl+C/interrupt architecture research completed; no premature signal dependency added.

### Current observations

- The parser intentionally supports only words, quotes, and escapes. Operators are blocked until a grammar and semantic model are designed.
- Native process execution is deliberately isolated from future shell-language features.
- Persistent history is opt-out and uses a centralized platform-aware user-state boundary; Unix history files are private 0600 files.
- Configuration is declarative TOML and currently supports only `[ui].prompt`; it never executes commands.
- Configuration defaults are overridden by the user configuration file. CLI/environment overrides are not yet implemented and must not be implied by documentation.
- Unknown configuration settings are rejected rather than silently ignored.
- Configuration loading has an explicit-path boundary, allowing deterministic file-loading tests without mutating the user's real environment.
- A deterministic testing strategy is documented in `docs/TESTING.md`.
- The current GitHub integration can write repository files, but no verified CI execution has been observed for the current main branch. Therefore CI is not considered validated.
- `Cargo.lock` is currently only a generated header with no resolved packages. It must not be described as a verified reproducible dependency lock until a real Cargo resolution/build has populated and validated it.
- A previous GitHub contents write was rejected because the remote blob SHA had changed; subsequent changes re-read the remote file before writing. This is now part of the safe-edit protocol.
- `src/main.rs` and `lib.rs` were reviewed together to ensure the binary's help/version entry points are exported consistently.
- Parser coverage explicitly includes empty input, whitespace separation, escaped quotes inside double quotes, and adjacent quoted/unquoted text. These tests are written but not yet execution-verified.
- Shell tests cover current-directory initialization, relative `cd`, invalid `cd` arity, missing directories, regular-file targets, and unknown commands. Temporary shell test paths now include a time-based nonce to reduce parallel-test collision risk. These tests are written but not yet execution-verified.
- Process tests cover missing executables, invalid program names, exit status, working-directory propagation, and missing working directories. The missing-working-directory tests use a known platform command so they prove the working-directory precondition rather than conflating it with program lookup failure. These tests are written but not yet execution-verified.
- The CLI now has executable-level smoke tests for help/version aliases, interactive `exit`, interactive help, and EOF. These tests are written but not yet execution-verified.
- Configuration load errors are intentionally recoverable at shell startup: Forge reports a warning and falls back to defaults rather than becoming unusable because of a malformed user config.
- Parser review found no need to broaden grammar during this loop. The implementation remains a deliberately small state machine with explicit errors for unfinished escapes and unterminated quotes.
- Parser syntax is not yet a stable promise beyond words, quotes, and escapes; future operators must not be bolted onto this tokenizer without an explicit grammar design.
- `Shell::run` treats Ctrl+C while idle as an input interruption and continues the REPL. A child process is launched synchronously through `Command::status`, so the exact foreground-signal behavior while a child is running is an OS/process-group concern and is not yet a promised Forge semantic.
- Research of the maintained `ctrlc` crate confirms it provides cross-platform Ctrl+C handlers, but installing a global handler would overwrite Unix signal dispositions and does not by itself establish correct child process-group/job-control semantics. It is therefore not being added merely to make the Ctrl+C box look complete. citeturn0search0turn0search1

### Decisions made in this loop

- Keep the CI gate defined but do not treat an absent GitHub Actions run as a passing gate.
- Do not create repeated no-op commits solely to provoke Actions.
- Preserve the current small process API until integration evidence shows a real need for another abstraction.
- Treat cross-module compile consistency as a required static review gate even when no executor is available.
- Reconcile remote file SHA before every sequential GitHub contents update.
- Expand parser and shell tests before expanding shell semantics; this reduces regression risk without prematurely committing Forge to shell-operator behavior.
- Keep `Shell::execute` private; test public behavior boundaries through small in-module invariants rather than exposing internals as a testing convenience.
- Keep configuration file I/O injectable through an explicit path rather than mutating process-wide environment variables in tests.
- Isolate temporary test paths from parallel collisions instead of relying on process ID alone.
- Treat malformed or unreadable user configuration as a recoverable startup condition; preserve defaults and surface a warning.
- Do not add shell operators, expansion, globbing, or scripting during parser hardening; establish the grammar and semantic model first.
- Parser hardening did not reveal a justified semantic expansion; preserve the small state-machine boundary until property/fuzz testing provides evidence for further changes.
- Do not add a Ctrl+C library solely for interactive interruption. Correct child interruption requires an explicit process-group/signal contract, cross-platform tests, and a decision about future job control. A dependency can be adopted later if it is part of that complete design.

## Reuse research already identified

- Brush: reference for parser/runtime separation, interactive behavior, testing, configuration, and platform concerns.
- ReShell: reference for parser/checker/runtime/builtins/REPL separation.
- DataDog rshell: reference for security and capability-oriented execution ideas, with different product goals.
- Conch: reference for project organization, packaging, and cross-platform engineering.
- `directories`: candidate for future platform path handling; not adopted yet.
- Nushell and fish: references for mature configuration and startup ordering; Forge deliberately avoids executable startup configuration for v0.1.
- `toml` + `serde`: selected for the current declarative configuration implementation after reuse review at the design level.
- `ctrlc` 3.5.x: researched for cross-platform Ctrl+C handling; not adopted because a global handler alone does not define correct child process/job-control semantics.

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
