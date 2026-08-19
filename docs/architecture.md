# Forge Architecture — v0.1 Foundation

## Scope

Forge v0.1 is an interactive command shell. It is **not** a terminal emulator and it does not attempt to replace an existing operating-system shell wholesale.

## Design principles

1. Keep the core native and independent of AI services.
2. Prefer standard-library OS primitives where they are sufficient.
3. Add a dependency only when it provides meaningful user value or removes substantial platform complexity.
4. Keep parsing, command dispatch, process execution, and terminal interaction separable.
5. Avoid speculative abstraction. A boundary earns its place when it protects a real invariant, enables testing, or has a credible second implementation.
6. Treat observable behavior as an API: errors, exit behavior, paths, and command arguments must remain predictable.

## Current flow

```text
terminal input
      |
      v
line editor
      |
      v
command parser
      |
      v
built-in dispatcher -----> built-in operation
      |
      +-------------------> native process execution
```

The current parser intentionally supports words, single quotes, double quotes, and escapes. It does **not** implement shell operators such as pipes, redirection, command substitution, boolean chaining, globbing, or job control.

## Why Rust

Rust provides a native executable, strong memory-safety guarantees, explicit operating-system APIs, a mature package/build system, and practical support for Windows, Linux, and macOS. It is a good foundation for a tool that may eventually need deeper process and system integration.

## Why rustyline

Forge should feel like a real interactive program rather than a loop around `stdin.read_line`. Rustyline 18 provides cross-platform line editing, command history, Unicode support, interruption handling, and completion-oriented infrastructure. It is isolated behind the shell's interactive loop so that a future terminal/input subsystem can replace it without redefining command semantics.

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
- persistent configuration format;
- Git integration;
- terminal emulator.

Postponing these is intentional. Each one creates observable semantics that can become difficult to change after users depend on them.

## Testing strategy

Pure command parsing is unit-tested independently from process execution. The CI quality gate runs formatting, Clippy with warnings denied, the test suite, and a release build.

Future increments should add integration tests around process execution and platform-specific behavior before expanding shell semantics.

## Architectural review rule

Before adding a major subsystem, answer:

- What user-visible problem does it solve?
- Can it be tested without a real terminal?
- What operating-system assumptions does it make?
- What security boundary does it create?
- Can it be replaced without rewriting unrelated code?
- What behavior becomes difficult to change once released?

If those questions cannot be answered clearly, the feature is not ready to enter the core.
