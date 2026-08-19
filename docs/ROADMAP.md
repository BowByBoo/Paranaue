# Forge Roadmap

This roadmap is intentionally capability-driven rather than date-driven.

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
- [x] Cross-platform CI quality gate
- [x] Architecture documentation
- [x] Modular shell/parser/process/history boundaries

## Current architectural loop

Before implementing shell language features, complete the reliability work:

- [ ] deterministic process-execution integration tests
- [ ] release/installation strategy
- [ ] configuration model and precedence rules
- [ ] command registry design
- [ ] exit-status model
- [ ] security review of environment and process inheritance
- [ ] UX review of prompt and diagnostics
- [ ] reproducible lockfile committed after a verified dependency resolution

## Shell language — architectural approval required

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

No item in the later sections is a promise. Each must earn its place through user value, technical feasibility, security review, platform testing, and maintainability.
