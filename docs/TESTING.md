# Forge testing strategy

Forge treats compilation as only one layer of evidence. A feature is complete only after the relevant unit, integration, platform, and user-facing behavior has been exercised.

## Test layers

### Unit tests

Use unit tests for deterministic logic that can be validated without launching a shell session. Current examples include parser behavior, configuration decoding, prompt expansion, and process error classification.

### Integration tests

Integration tests are reserved for behavior at module boundaries: spawning a real process, propagating its exit status, changing directories, loading configuration, and preserving recoverable errors.

Tests should prefer deterministic, platform-native fixtures over assumptions about optional programs installed on a developer machine.

### Platform tests

The supported target ambition is Windows, Linux, and macOS. Platform-sensitive behavior must be tested on the relevant runner rather than inferred from another operating system.

### Release smoke tests

Before Forge v0.1 can be called usable, a release artifact must be built and exercised on a clean supported environment. The smoke test must prove that the binary starts, accepts input, executes a deterministic native command, handles a missing command, and exits cleanly.

## Determinism rules

- Do not depend on network access.
- Do not depend on user-specific paths.
- Do not assume Git, Python, Node, or another optional tool is installed.
- Prefer commands available on the target operating system or a Forge-controlled test fixture.
- Avoid timing-based assertions where a deterministic exit status or file fixture can be used.
- Keep tests isolated from the developer's real command history and configuration.

## Failure protocol

When a bug is discovered:

1. Reproduce it deterministically.
2. Add the smallest test that fails for the bug.
3. Fix the implementation.
4. Run the relevant test layer again.
5. Run the broader regression suite.
6. Record the architectural lesson in `docs/ROADMAP.md` when it affects future work.

## CI is evidence

A workflow file is not evidence that CI passed. The project may claim a CI gate only after a real GitHub Actions run reports success for the relevant commit.

## Current limitation

The connected GitHub environment can edit repository files, but this conversation does not provide a local Cargo runtime. Therefore no response may claim that Rust tests passed unless an actual Cargo execution or GitHub Actions result has been observed.
