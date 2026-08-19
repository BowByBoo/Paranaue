# Forge Roadmap

This roadmap is intentionally capability-driven rather than date-driven.

## Foundation — v0.1

- [x] Native Rust application
- [x] Interactive line editing
- [x] Session command history
- [x] Built-in help
- [x] Built-in version
- [x] Working-directory navigation
- [x] Native process execution
- [x] Structured argument parsing for words, quotes, and escapes
- [x] Actionable command errors
- [x] Unit tests for parser edge cases
- [x] CI quality gate
- [x] Architecture documentation

## Next architectural loop

Before implementing these, revisit the architecture with the full review loop:

- [ ] persistent configuration model
- [ ] persistent history policy
- [ ] command registry abstraction
- [ ] integration tests for process execution
- [ ] cross-platform CI matrix
- [ ] installation/package strategy
- [ ] shell operator design

## Later — only after architectural approval

- [ ] pipes
- [ ] redirection
- [ ] command chaining
- [ ] environment expansion
- [ ] globbing
- [ ] job control
- [ ] completion
- [ ] plugins
- [ ] scripting
- [ ] Git-aware tooling

No item in the later section is a promise. Each must earn its place through user value, technical feasibility, security review, and maintainability.
