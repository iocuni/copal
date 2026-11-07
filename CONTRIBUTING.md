# Contributing

Bug reports, feature proposals, and pull requests are welcome.

Before opening a pull request, please:

1. Keep changes focused and explain user-visible behavior changes.
2. Run `cargo fmt --all -- --check`.
3. Run `cargo test --locked`.
4. Run `cargo clippy --locked --all-targets -- -D warnings`.
5. Run `cargo build --locked --release`.
6. To create installable VST3 and CLAP bundles, run `cargo xtask bundle copal --release`.

Please include tests when changing DSP behavior. Avoid adding dependencies unless
they are needed for the change.
