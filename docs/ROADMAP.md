# Forge Roadmap

This roadmap is the project's durable engineering memory. Every architectural loop must update it before moving on. Dates are deliberately avoided; quality and evidence determine progression.

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
- [x] Configuration architecture proposal documented before implementation

## Current LOOP — reliability before language expansion

The project is **not yet declared usable**. The current loop is focused on proving the foundation rather than adding visible features.

### In progress

- [ ] Obtain and verify a real CI run on the current main branch.
- [ ] Deterministic process-execution integration tests.
- [ ] Verify the dependency lockfile after a real Cargo resolution/build.
- [ ] Release and installation strategy for a clean machine.
- [ ] Implement the approved configuration model and precedence rules.
- [ ] Explicit exit-status model and semantics.
- [ ] Security review of environment and process inheritance.
- [ ] UX review of prompt, diagnostics, interruption, and EOF behavior.
- [ ] Clean-machine installation smoke test.
- [ ] Red Team review of the complete current codebase.
- [ ] Research mature shell projects and crates before expanding the shell language.

### Current observations

- The current parser intentionally supports only words, quotes, and escapes. Operators are blocked until a grammar and semantic model are designed.
- Native process execution is deliberately isolated from future shell-language features.
- Persistent history is opt-out and uses a centralized platform-aware user-state boundary; Unix history files are private 0600 files.
- The current GitHub integration can write repository files, but a verified CI execution has not yet been observed for the current main branch. Therefore CI is not considered validated.
- `Cargo.lock` is currently only a generated header with no resolved packages. It must not be described as a verified reproducible dependency lock until a real Cargo resolution/build has populated and validated it.
- The CI workflow currently regenerates the lockfile in the runner before quality gates. This proves dependency resolution is possible during CI, but it does not by itself make the repository lockfile reproducible; that distinction remains explicit.
- Configuration is not implemented yet. Its architecture is documented separately in `docs/CONFIGURATION.md`.
- No shell-language operators are being implemented until their grammar and semantics have received a full architectural review.

### Decisions made in this loop

- Do not manually fabricate or partially reconstruct `Cargo.lock`. A real Cargo resolution must produce it; the resulting lockfile will then be reviewed and committed as a coherent change.
- Do not treat a workflow definition as evidence of a successful build. CI is validated only by an observed successful run on the relevant commit.
- Do not introduce an executor trait merely to make process tests easier while there is only one process implementation. Prefer integration tests or a second real implementation pressure before adding that abstraction.
- Process execution remains an OS-native boundary. Pipes, redirection, chaining, expansion, globbing, and job control do not belong in this layer.
- The shell parser currently understands only words, quotes, and escapes. Operators require a separately designed grammar before implementation.
- Persistent history is opt-out and is stored using platform-appropriate user state locations. On Unix, the history file is created with private 0600 permissions.
- `Ctrl+C` interrupts the current input line rather than terminating the entire Forge session; EOF exits the session.
- The terminal emulator is explicitly outside the v0.1 product boundary.
- Substantial infrastructure must pass the Reuse Before Reinvent review before Forge implements it from scratch.
- Forge has a small `paths` boundary for platform-aware user state. Configuration must have its own configuration location rather than reusing state paths.
- Forge v0.1 configuration will be declarative rather than executable. Startup configuration will not be a hidden scripting engine.
- The initial configuration precedence is intended to be built-in defaults, then user configuration, then explicitly documented command-line/environment overrides. Project-local configuration is deferred until a trust model exists.

### Reuse research already identified

- Brush is a mature Rust shell implementation worth studying for parser/runtime separation, interactive behavior, testing, configuration, and platform concerns.
- ReShell is useful as a reference for separating parser, checker, runtime, builtins, and REPL responsibilities.
- DataDog rshell is worth studying for security and capability-oriented execution ideas, while recognizing that its product goals differ from Forge.
- Conch is useful as a reference for project organization, packaging, and cross-platform engineering.
- The `directories` Rust crate provides cross-platform application config/data/state locations and remains a candidate for the future configuration boundary; it has not been adopted yet.
- Nushell and fish demonstrate mature configuration systems with user configuration, platform-aware paths, and startup ordering. Forge is deliberately not copying their executable-configuration model for v0.1 because it would couple configuration to an as-yet undefined shell language.

These projects are references, not automatic dependencies. Each future adoption decision requires its own license, maintenance, security, portability, performance, and architectural review.

## Shell language — architectural approval required

These features remain blocked until the grammar, semantics, security model, UX, portability, and tests have been reviewed by the full LOOP.

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
- [ ] configuration behavior is documented
- [ ] installation works on a clean supported machine
- [ ] release artifact can be identified and reproduced
- [ ] security review has no unresolved high-severity findings
- [ ] Red Team has attempted to break the MVP
- [ ] UX review has no known critical usability failures
- [ ] documentation matches the shipped behavior
- [ ] architecture review finds no known severe foundational flaw

Until these gates are met, the status remains **IN DEVELOPMENT**.

## LOOP protocol

For every meaningful change:

`ANALYZE → RESEARCH → PROPOSE → CRITICIZE → SIMPLIFY → IMPLEMENT → TEST → RED TEAM → CORRECT → REVIEW → DOCUMENT → UPDATE ROADMAP → COMMIT → REPEAT`

If a new uncertainty could cause major structural rework, stop and resolve it before implementation. Never mark a feature complete solely because it compiles.
