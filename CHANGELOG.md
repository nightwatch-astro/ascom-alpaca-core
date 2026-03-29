# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1](https://github.com/nightwatch-astro/ascom-alpaca-core/compare/v0.2.0...v0.2.1) - 2026-03-29

### Ci

- skip semver-check when no Rust code changes ([#101](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/101))
- add dependabot config and patch auto-merge ([#99](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/99))

## [0.2.0](https://github.com/nightwatch-astro/ascom-alpaca-core/compare/v0.1.4...v0.2.0) - 2026-03-29

### Features

- *(ci)* add id-token permission for trusted publishing ([#96](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/96))

### Miscellaneous

- add .gitattributes for linguist-generated patterns ([#98](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/98))
- *(ci)* cache ConformU, parallelize jobs, reusable release workflow ([#93](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/93))

## [0.1.4](https://github.com/nightwatch-astro/ascom-alpaca-core/compare/v0.1.3...v0.1.4) - 2026-03-28

### Performance

- *(ci)* per-device ConformU settings, telescope split, reduced timeouts ([#92](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/92))

## [0.1.2](https://github.com/nightwatch-astro/ascom-alpaca-core/compare/v0.1.1...v0.1.2) - 2026-03-28

### Refactoring

- macros, DeviceStateBuilder, clippy cleanup ([#87](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/87))

## [0.1.1](https://github.com/nightwatch-astro/ascom-alpaca-core/compare/v0.1.0...v0.1.1) - 2026-03-28

### Bug Fixes

- *(ci)* conformu-filter fetch base ref before diff ([#76](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/76))

### Miscellaneous

- *(ci)* bump amannn/action-semantic-pull-request from 5 to 6 ([#81](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/81))
- *(ci)* bump actions/cache from 4 to 5 ([#86](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/86))
- *(ci)* bump actions/setup-node from 4 to 6 ([#85](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/85))
- *(ci)* bump actions/upload-artifact from 4 to 7 ([#83](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/83))
- *(ci)* bump actions/create-github-app-token from 2 to 3 ([#80](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/80))
- *(ci)* bump actions/checkout from 4 to 6 ([#82](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/82))

### Performance

- *(ci)* remove Cargo.toml and examples from ConformU shared patterns ([#84](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/84))
- *(ci)* rust-cache, semver-checks as CI job with official action ([#78](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/78))

## [0.1.0](https://github.com/nightwatch-astro/ascom-alpaca-core/releases/tag/v0.1.0) - 2026-03-28

### Bug Fixes

- *(ci)* remove pre_release_hook, use cargo add in README instead of pinned version ([#74](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/74))
- *(ci)* release-plz config — semver_check, dependencies_update, features_always_increment_minor, pr_labels, pre_release_hook under [[package]] ([#73](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/73))
- *(ci)* use app token for release-please ([#72](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/72))
- *(ci)* add contents:write and pull-requests:write permissions for release-please ([#69](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/69))

### Features

- [**breaking**] ConformU V4 compliance, docs, CI, release-please ([#67](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/67))

### Miscellaneous

- scaffold crate — Cargo.toml, dual license, CI

### Performance

- *(ci)* fast checks on every push, ConformU only on PRs, cancel duplicates ([#70](https://github.com/nightwatch-astro/ascom-alpaca-core/pull/70))
