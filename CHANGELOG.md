# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-19

### Added

- Parse BNP Paribas bank statements (PDF) and extract operations to YAML
- Extract `date_du_releve` from statement header
- Compute correct year for operations spanning year boundary
- Use `chrono::NaiveDate` for date fields
- CLI tool (`mybnp`) and library
- GitHub Actions CI workflow
- GitHub Actions publish workflow for crates.io
- Tests for year computation

[0.1.0]: https://github.com/laurentcarrie/bnp/releases/tag/v0.1.0
