# CI validation status

## Current status

Forge has a GitHub Actions quality workflow targeting Linux, Windows, and macOS. The workflow performs formatting, Clippy, tests, and a release build.

As of the current engineering loop, GitHub has not exposed a workflow run associated with the recent `main` commits. Therefore the project does **not** claim that CI, tests, or release builds have passed.

## Required evidence

A future loop may mark CI validated only after an actual GitHub Actions run is observed and all matrix jobs report success.

Required checks:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets --all-features`
- `cargo build --release`

The matrix must cover Linux, Windows, and macOS.

## Investigation rule

Do not create repeated no-op commits merely to trigger Actions. If no run appears, investigate repository/workflow configuration or use a different validated execution path. Never infer success from the existence of the workflow YAML.
