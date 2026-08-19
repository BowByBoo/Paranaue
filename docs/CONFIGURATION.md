# Forge Configuration — v0.1 Contract

Status: **implemented, awaiting CI verification**

Forge v0.1 uses a **declarative TOML configuration file**. Configuration is data, not startup code.

## Current schema

```toml
[ui]
prompt = "forge {cwd}> "
```

The only setting currently supported is `ui.prompt`.

`{cwd}` is replaced by Forge's current working directory. No other template language is interpreted.

Unknown sections and settings are rejected instead of being silently ignored.

## Location

Forge keeps configuration separate from persistent state.

- Linux/Unix: `$XDG_CONFIG_HOME/forge/config.toml`, falling back to `$HOME/.config/forge/config.toml`.
- macOS: `$HOME/Library/Application Support/Forge/config.toml`.
- Windows: `%APPDATA%/Forge/config.toml`.

These conventions are implemented in the platform path boundary and must be verified by CI and integration tests.

## Precedence

The current precedence is intentionally simple:

1. built-in defaults;
2. user configuration file.

Command-line/environment overrides are **not implemented yet** and must not be implied by documentation.

Project-local configuration is deferred until a concrete use case and trust model exist.

## Failure behavior

If the configuration file exists but cannot be read or parsed, Forge fails startup with an actionable error containing the configuration path and the parser/read failure.

Malformed configuration is not silently ignored.

## Security boundary

Configuration never executes commands, expands shell syntax, or writes files while being parsed. It is treated as untrusted user-controlled data.

The prompt currently performs only a literal `{cwd}` substitution after successful deserialization.

## Reuse decision

The implementation uses the mature `toml` and `serde` Rust ecosystem rather than introducing a custom configuration parser. The selected crates must still be validated through Forge's CI and dependency review.

## Deferred

- additional settings;
- environment-variable overrides;
- command-line configuration flags;
- project-local configuration;
- configuration migrations/versioning;
- executable startup configuration.

These remain separate architectural decisions and must pass the full LOOP before implementation.
