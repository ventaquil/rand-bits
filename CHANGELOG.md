# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Use `Rng::gen_range` to generate the mapping index.
- Use `u32` for the `bits` argument.
- Split each type into high/low parts.
- Do not rand when the min/max value is wanted.
- Cut the mapping struct by half.

## [0.0.0] - 2023-12-08

### Added

- Initial release.

[Unreleased]: https://github.com/ventaquil/rand-bits/compare/v0.0.0...HEAD
[0.0.0]: https://github.com/ventaquil/rand-bits/releases/tag/v0.0.0
