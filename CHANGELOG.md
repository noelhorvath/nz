# Changelog

All notable changes to `nz` crate will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].

## [0.3.5] - 2024-05-02

### Fixed

- Use crate version in `create.io` and `doc.rs` badges
- Use `main` branch in `CI` badge

## [0.3.4] - 2024-05-01

### Added

- New deploy workflow

### Changed

- Add new line after import in `Usage` example code
- Capitalize link names under `<!-- Links -->` in `CHANGELOG.md`
- Capitalize first latter of each list item in `CHANGELOG.md`
- Rename `rust.yml` to `check.yml` in `.github/workflows
- Update and improve `Check` workflow
- Update links to versions in `CHANGELOG.md`

### Fixed

- Use `1.56.0` version of rust API docs instead of `stable`
- Deploy and check workflows

## [0.3.3] - 2024-03-08

### Changed

- Use `->` instead of `=>` in comment when representing an arrow
- Remove unnecessary `-` after `NonZero` in crate documentation

### Fixed

- `Zlib license` link under `Licenses` section
- Grammar errors in `changelog.md`

### Removed

- `let nz = nz::i8!(27);` from the first macro example

## [0.3.2] - 2023-10-16

### Changed

- Reorder subsections in `0.3.1` (`changelog.md`)
- Prefix fully-qualified `NonZero` paths with `::`
- Correct/improve comments
- Crate-level documentation
    - Rewrite `Limitations` section
    - Sync `README.md` with crate-level doc in `lib.rs`
    - Improve example code in `Usage`
    - Update `Features` section
    - Rename `Non-Zero macros` section to `Macros`

### Fixed

- List under `Compilation errors` in generated macro documentation

### Removed

- Unnecessary link after [`core::num`] in `lib.rs`

## [0.3.1] - 2023-10-11

### Changed

- Recategorize `[0.2.0] - increase MSRV from 1.47.0 to 1.56.0` as non-breaking change
  in `changelog.md`
- Add documentation generation to `gen_non_zero_macros`

### Removed

- `publish-crate` workflow
- Non-generated documentation for each generated macro

## [0.3.0] - 2023-09-14

### Added

- Configure rust toolchain in `rust-toolchain.toml`
- `publish-crate` workflow for publishing crate
- `Changelog` section with link to `changelog.md`
- Add dummy example for `ZeroIsInvalidForNonZero` never type

### Changed

- Rename `Basic usage` section to `Usage` in both `README.md` and `lib.rs`
- Remove trailing `.` from crate description
- Update `rust` workflow
- Update old macro const name to new in docs and comments
- Put all types between backticks in `NonZero macros` section in `lib.rs`

#### Breaking changes

- Rename inner macro const to comply with `non-upper-case-globals` lint

### Fixed

- Correct repository link for `github` badge
- Correct link to `rust` workflow for `rust-ci` badge
- `license` badge in `README.md`
- Comply with all doc lints
- Comply with `non-upper-case-globals` warning

## [0.2.2] - 2023-07-30

### Changed

- Update crate description
- Directly include documentation instead of including from `README.md` in `lib.rs`
- Make badges declaration more readable in `README.md`

## [0.2.1] - 2023-07-30

### Fixed

- Correct typos in each macro documentation

## [0.2.0] - 2023-07-30

### Changed

- Bump rust edition from `2018` to `2021`
- Change `MSRV` for `test` job in `rust.yml`
- Merge `README.md` with crate documentation in `lib.rs`
- Include documentation for crate from `README.md`
- Update badges in `README.md`
- Increase `MSRV` from `1.47.0` to `1.56.0`

## [0.1.4] - 2023-07-29

### Changed

- Update crate documentation
- Update `Limitations`
- Remove asserts commented asserts from examples in `README.md`

### Fixed

- Correct code comments in `Basic usage`
- Correct `doc.rs` links to macro types in `Macros` section in `README.md`

## [0.1.3] - 2023-07-29

### Added

- `Limitations` in documentation and `README.md`
- New documentation tests and examples

### Changed

- Update documentation tests and examples
- Update crate description
- Synchronize crate documentation with `README.md`
- Improve badges in `README.md`

#### Breaking changes

- Reduce possible const dependency cycles to minimal

## [0.1.2] - 2023-07-29

### Added

- Improve `README.md`
    - Add `rust` GitHub workflow badge
    - Add `docs.rs` badge
    - Add `crates.io` badge
    - Add `unsafety` badge
    - Add `license` badge
    - Add link to `core::num` references

### Changed

- Update `README.md`
    - Correct sentence in `REMARKS` section

## [0.1.1] - 2023-07-29

### Changed

- Update `Cargo.toml`
    - Rename category name `no_std` to `no-std`

## [0.1.0] - 2023-07-29

- Initial release

<!-- Links -->
[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

<!-- Versions -->
[0.3.5]: https://github.com/noelhorvath/nz/compare/v0.3.4...v0.3.5
[0.3.4]: https://github.com/noelhorvath/nz/compare/v0.3.3...v0.3.4
[0.3.3]: https://github.com/noelhorvath/nz/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/noelhorvath/nz/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/noelhorvath/nz/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/noelhorvath/nz/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/noelhorvath/nz/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/noelhorvath/nz/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/noelhorvath/nz/compare/v0.1.4...v0.2.0
[0.1.4]: https://github.com/noelhorvath/nz/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/noelhorvath/nz/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/noelhorvath/nz/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/noelhorvath/nz/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/noelhorvath/nz/compare/b165aa5...v0.1.0
