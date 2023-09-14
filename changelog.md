# Changelog

All notable changes to `nz` crate will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].

## [0.3.0] - 2023-09-14

### Added

- configure rust toolchain in `rust-toolchain.toml`
- `publish-crate` workflow for publishing crate
- `Changelog` section with link to `changelog.md`

### Changed

- rename `Basic usage` section to `Usage` in both `README.md` and `lib.rs`
- remove trailing `.` from crate description
- update `rust` workflow

### Fixed

- correct repository link for `github` badge
- correct link to `rust` workflow for `rust-ci` badge
- `license` badge in `README.md`

## [0.2.2] - 2023-07-30

### Changed

- update crate description
- directly include documenation instead of including from `README.md` in `lib.rs`
- make badges declaration more readable in `README.md`

## [0.2.1] - 2023-07-30

### Fixed

- correct typos in each macro documentation

## [0.2.0] - 2023-07-30

### Changed

- update `Cargo.toml`
    - increase `MSRV` from `1.47.0` to `1.56.0`
    - bump rust edition from `2018` to `2021`
- change `MSRV` for `test` job in `rust.yml`
- merge `README.md` with crate documentation in `lib.rs`
- include documentation for crate from `README.md`
- update badges in `README.md`


## [0.1.4] - 2023-07-29

### Changed

- update crate documentation
- update `Limitations`
- remove asserts commented asserts from exmaples in `README.md`

### Fixed

- correct code comments in `Basic usage`
- correct `doc.rs` links to macro types in `Macros` section in `README.md`

## [0.1.3] - 2023-07-29

### Added

- `Limitations` in documentation and `README.md`
- new documentation tests and examples

### Changed

- update documentation tests and exmaples
- update crate description
- synchronize crate documentation with `README.md`
- improve badges in `README.md`

### Fixed

- reduce possible const dependency cycles to minimal

## [0.1.2] - 2023-07-29

### Added

- improve `README.md`
    - add `rust` GitHub workflow badge
    - add `docs.rs` badge
    - add `crates.io` badge
    - add `unsafety` badge
    - add `license` badge
    - add link to `core::num` references

### Changed

- update `README.md`
    - correct sentence in `REMARKS` section

## [0.1.1] - 2023-07-29

### Changed

- update `Cargo.toml`
    - rename category name `no_std` to `no-std`

## [0.1.0] - 2023-07-29

- initial release

<!-- Links -->
[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html

<!-- Versions -->
[0.3.0]: https://github.com/noelhorvath/nz/compare/6a7e28d...HEAD
[0.2.2]: https://github.com/noelhorvath/nz/compare/452838d...6a7e28d
[0.2.1]: https://github.com/noelhorvath/nz/compare/3e63b92...452838d
[0.2.0]: https://github.com/noelhorvath/nz/compare/1560ec0...3e63b92
[0.1.4]: https://github.com/noelhorvath/nz/compare/ae37c3d...1560ec0
[0.1.3]: https://github.com/noelhorvath/nz/compare/460d3f3...ae37c3d
[0.1.2]: https://github.com/noelhorvath/nz/compare/0f080b9...460d3f3
[0.1.1]: https://github.com/noelhorvath/nz/compare/b67c25a...0f080b9
[0.1.0]: https://github.com/noelhorvath/nz/compare/b165aa5...b67c25a