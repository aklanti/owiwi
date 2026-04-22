set unstable

export RUST_LOG := env("RUST_LOG", "debug")

alias t := test

[doc('Format all code (pass --check to verify)')]
[group('dev')]
@fmt *args: (rustfmt args) (sort args) (justfmt args)

[doc('Run tests')]
[group('dev')]
@test *args:
    cargo nextest run --all-features --all-targets {{ args }}
    cargo test --doc

[doc('Run test coverage')]
[group('dev')]
cov *args:
    cargo llvm-cov clean
    cargo llvm-cov nextest --all-features {{ args }}

[doc('Run all linters and format checks')]
[group('check')]
@lint: clippy deny hack shear (fmt "--check")

[doc('Lint all code')]
[group('check')]
@clippy:
    cargo clippy --all-features --all-targets

[doc('Check dependency rules')]
[group('check')]
@deny:
    cargo deny check

[doc('Check feature flag combinations')]
[group('check')]
@hack:
    cargo hack --feature-powerset check

[doc('Check for unused dependencies')]
[group('check')]
@shear *args:
    cargo shear {{ args }}

[doc('Build documentation')]
[group('build')]
@doc *args:
    cargo doc --no-deps --all-features {{ args }}

[doc('Install workspace tools')]
[private]
install-tools:
    cargo install cargo-binstall
    cargo binstall --no-confirm cargo-deny
    cargo binstall --no-confirm cargo-hack
    cargo binstall --no-confirm cargo-llvm-cov
    cargo binstall --no-confirm cargo-nextest
    cargo binstall --no-confirm cargo-shear
    cargo binstall --no-confirm cargo-sort

[private]
@rustfmt *args:
    cargo +nightly fmt --all {{ if args == "--check" { "-- --check" } else { "" } }}

[private]
@sort *args:
    cargo sort --grouped {{ args }}

[private]
@justfmt *args:
    just --fmt {{ args }}
