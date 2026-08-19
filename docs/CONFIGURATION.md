# Forge Configuration — Design Decision

Status: **proposed for v0.1**

This document records the configuration boundary before implementation. The goal is to avoid coupling Forge's configuration format to the future shell language.

## Decision

Forge v0.1 will use a **declarative configuration file** rather than executing arbitrary startup code.

The configuration layer must not become a hidden scripting engine.

The initial configuration model should contain only settings that Forge itself owns, such as:

- prompt presentation;
- history behavior and limits;
- interactive editing preferences;
- diagnostic/output preferences that are proven useful.

Shell aliases, functions, plugins, executable startup hooks, and arbitrary command execution are explicitly outside this first configuration contract.

## Why

Mature shells demonstrate several valid approaches. Brush currently uses TOML configuration, while Nushell and fish use executable configuration/startup files with richer semantics. Those designs are useful references, but executable configuration would couple configuration to a language/runtime that Forge has not yet designed. It would also expand the security and startup-time surface prematurely.

Forge should therefore start with data, not code.

## Location

The path layer will distinguish configuration from persistent state.

Platform conventions will be respected rather than hard-coded into the shell. On Unix-like systems, the design should honor XDG configuration conventions. On macOS and Windows, the corresponding per-user application configuration locations should be used.

The exact platform mapping will be implemented and tested as part of the configuration feature; this document intentionally does not duplicate platform-specific path logic.

## Precedence

The initial precedence model is intentionally small:

1. built-in defaults;
2. user configuration file;
3. explicit command-line options/environment overrides, but only where the setting has a documented override mechanism.

Forge must not silently merge arbitrary configuration sources.

Project-local configuration is deferred until there is a concrete use case and a defined trust model.

## Failure behavior

Invalid configuration must never silently change behavior.

The user should receive:

- the configuration path;
- the failing setting or location when known;
- a useful explanation;
- a non-zero startup failure only when the invalid configuration prevents safe operation.

Forge should prefer a clear error over silently ignoring malformed configuration.

## Security boundary

Configuration is user-controlled input.

The parser must:

- reject malformed values safely;
- avoid executing commands;
- avoid arbitrary filesystem writes during parsing;
- avoid following unexpected paths merely because they appear in configuration;
- never interpret configuration as shell source.

## Reuse review

Before implementing the format, the team must compare a small hand-rolled format against mature Rust TOML/serialization crates. The chosen dependency must pass the project's Reuse Before Reinvent review: license, maintenance, security, portability, performance, dependency cost, and replaceability.

## Deferred decisions

The following are intentionally unresolved until the implementation review:

- exact file name;
- exact serialization crate(s);
- complete setting schema;
- environment-variable override naming;
- command-line configuration flags;
- project-local configuration;
- configuration migrations/versioning.

No feature should depend on these unresolved details until the architecture review approves them.
