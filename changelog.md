# Changelog

All notable changes to `nz` crate will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].

## [0.3.2] - 2023-10-16

### Changed

- reorder subsections in `0.3.1` (`changelog.md`)
- prefix fully-qualified `NonZero` paths with `::`
- correct/improve comments
- crate-level documentation
    - rewrite `Limitations` section
    - sync `README.md` with crate-level doc in `lib.rs`
    - improve example code in `Usage`
    - update `Features` section
    - rename `Non-Zero macros` section to `Macros`

### Fixed

- list under `Compilation errors` in generated macro documentation

### Removed

- unnecessary link after [`core::num`] in `lib.rs`

## [0.3.1] - 2023-10-11

### Changed

- recategorize `[0.2.0] - increase MSRV from 1.47.0 to 1.56.0` as non-breaking change
  in `changelog.md`
- add documentation generation to `gen_non_zero_macros`

### Removed

- `publish-crate` workflow
- non-generated documentation for each generated macro

## [0.3.0] - 2023-09-14

### Added

- configure rust toolchain in `rust-toolchain.toml`
- `publish-crate` workflow for publishing crate
- `Changelog` section with link to `changelog.md`
- add dummy example for `ZeroIsInvalidForNonZero` never type

### Changed

- rename `Basic usage` section to `Usage` in both `README.md` and `lib.rs`
- remove trailing `.` from crate description
- update `rust` workflow
- update old macro const name to new in docs and comments
- put all types between backticks in `NonZero macros` section in `lib.rs`

#### Breaking changes

- rename inner macro const to comply with `non-upper-case-globals` lint

### Fixed

- correct repository link for `github` badge
- correct link to `rust` workflow for `rust-ci` badge
- `license` badge in `README.md`
- comply with all doc lints
- comply with `non-upper-case-globals` warning

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

- bump rust edition from `2018` to `2021`
- change `MSRV` for `test` job in `rust.yml`
- merge `README.md` with crate documentation in `lib.rs`
- include documentation for crate from `README.md`
- update badges in `README.md`
- increase `MSRV` from `1.47.0` to `1.56.0`

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

#### Breaking changes
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
[0.3.2]: https://github.com/noelhorvath/nz/compare/ac06516...HEAD
[0.3.1]: https://github.com/noelhorvath/nz/compare/6218e0c...ac06516
[0.3.0]: https://github.com/noelhorvath/nz/compare/6a7e28d...6218e0c
[0.2.2]: https://github.com/noelhorvath/nz/compare/452838d...6a7e28d
[0.2.1]: https://github.com/noelhorvath/nz/compare/3e63b92...452838d
[0.2.0]: https://github.com/noelhorvath/nz/compare/1560ec0...3e63b92
[0.1.4]: https://github.com/noelhorvath/nz/compare/ae37c3d...1560ec0
[0.1.3]: https://github.com/noelhorvath/nz/compare/460d3f3...ae37c3d
[0.1.2]: https://github.com/noelhorvath/nz/compare/0f080b9...460d3f3
[0.1.1]: https://github.com/noelhorvath/nz/compare/b67c25a...0f080b9
[0.1.0]: https://github.com/noelhorvath/nz/compare/b165aa5...b67c25a