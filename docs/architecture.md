# Forge Architecture — v0.1 Foundation

## Scope

Forge v0.1 is an interactive command shell. It is **not** a terminal emulator and it does not attempt to replace an existing operating-system shell wholesale.

## Design principles

1. Keep the core native and independent of AI services.
2. Prefer standard-library OS primitives where they are sufficient.
3. Add a dependency only when it provides meaningful user value or removes substantial platform complexity.
4. Keep parsing, command dispatch, process execution, history, and terminal interaction separable.
5. Avoid speculative abstraction. A boundary earns its place when it protects a real invariant, enables testing, or has a credible second implementation.
6. Treat observable behavior as an API: errors, exit behavior, paths, and command arguments must remain predictable.
7. Prefer small, reviewable commits over large rewrites.

## Current flow

```text
terminal input
      |
      v
line editor + history
      |
      v
command parser
      |
      v
built-in dispatcher -----> built-in operation
      |
      +-------------------> native process execution
```

The source is deliberately split into small modules:

- `parser.rs` owns command tokenization.
- `process.rs` owns native child-process execution.
- `history.rs` owns persistent interactive history and its platform-specific location.
- `shell.rs` owns the interactive lifecycle and built-in dispatch.
- `main.rs` is only the application entry point.

The parser intentionally supports words, single quotes, double quotes, and escapes. It does **not** implement shell operators such as pipes, redirection, command substitution, boolean chaining, globbing, or job control.

## Why Rust

Rust provides a native executable, strong memory-safety guarantees, explicit operating-system APIs, a mature package/build system, and practical support for Windows, Linux, and macOS. It is a good foundation for a tool that may eventually need deeper process and system integration.

## Why rustyline

Forge should feel like a real interactive program rather than a loop around `stdin.read_line`. Rustyline provides cross-platform line editing, command history, Unicode support, interruption handling, and completion-oriented infrastructure. It is isolated behind the shell's interactive loop so that a future terminal/input subsystem can replace it without redefining command semantics.

Forge now persists history in a platform-appropriate application state location when one can be determined. Failure to load or save history is deliberately non-fatal: a damaged or unwritable history file must never prevent the shell from starting.

## Error philosophy

User-facing command failures are reported without terminating the Forge process. Initialization failures that prevent the interactive shell from operating remain fatal. This distinction keeps ordinary mistakes (`cd` to a missing directory, unknown executable, non-zero child status) recoverable while still surfacing genuine startup failures.

Forge does not invoke an operating-system shell to execute arbitrary input. It parses the command into a program name and argument vector, then invokes the native process API directly. This makes the initial execution model explicit and avoids silently inheriting another shell's parsing semantics.

## Deliberately postponed

The following are not part of the foundation until their semantics are designed and tested:

- pipes;
- input/output redirection;
- command chaining (`&&`, `||`, `;`);
- background jobs and job control;
- glob expansion;
- environment expansion;
- command substitution;
- plugin ABI;
- scripting language;
- persistent user configuration format;
- Git integration;
- terminal emulator.

Postponing these is intentional. Each one creates observable semantics that can become difficult to change after users depend on them.

## Testing strategy

The parser is unit-tested independently from terminal I/O. The quality gate also runs formatting, Clippy with warnings denied, the test suite, and a release build. Persistent history is treated as best-effort infrastructure rather than a prerequisite for command execution.

Before adding shell semantics, future increments should add deterministic integration tests around process execution and platform-specific behavior. Where practical, child-process tests should use a platform-neutral helper rather than depending on a user's installed shell or command set.

## Architectural review rule

Before adding a major subsystem, answer:

- What user-visible problem does it solve?
- Can it be tested without a real terminal?
- What operating-system assumptions does it make?
- What security boundary does it create?
- Can it be replaced without rewriting unrelated code?
- What behavior becomes difficult to change once released?

If those questions cannot be answered clearly, the feature is not ready to enter the core.
