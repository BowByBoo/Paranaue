# Forge

Forge is the first brick of a long-term computing project: a native, independent command shell designed to grow for years without depending on AI at runtime.

## Current status

**v0.1 foundation — active development; not yet declared usable.**

The current foundation provides:

- native Rust executable;
- interactive prompt;
- built-in `help`, `pwd`, `cd`, `version`, and `exit` commands;
- native external process execution;
- quoted and escaped command arguments;
- Unicode-safe command input;
- actionable runtime errors;
- a growing automated test suite;
- a GitHub Actions quality-gate workflow defined for Linux, Windows, and macOS.

**Validation status:** the repository currently contains the test suite and CI workflow, but the project has not yet recorded a verified CI run for the current `main` branch. Therefore Forge is not yet claiming that its tests, Clippy checks, formatting checks, or release builds have passed in CI.

Forge deliberately does **not** attempt to be a Bash, PowerShell, Zsh, or terminal emulator replacement yet. Shell operators such as pipes, redirection, command chaining, globbing, and job control are intentionally outside the first foundation until their architecture has been designed and tested.

## Build

Install the stable Rust toolchain, then run:

```bash
cargo test
cargo run
```

For a release build:

```bash
cargo build --release
```

A clean-machine installation and release process are still part of the v0.1 completion gates.

## Philosophy

Forge is being developed around a simple rule:

> Build a foundation that still makes sense if the project is maintained for five years.

AI may be used as an external engineering aid during development, but Forge itself has no runtime dependency on OpenAI, ChatGPT, Ollama, language models, or AI services.

## Quality bar

Every meaningful feature is expected to go through analysis, implementation, automated testing, adversarial review, correction, documentation, and architectural review before it is considered complete. A written test is not treated as evidence that the test passed until it has actually executed.
