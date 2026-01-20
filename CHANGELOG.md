# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-01-20

### Added

- New CLI tool `my-bnp-add-patterns` to interactively categorize unassigned operations
  - Keyword-based category suggestions (restaurants, supermarkets, transport, etc.)
  - Pattern extraction from operation descriptions
  - Auto-accept mode (`a` key) to accept all suggestions automatically
  - Create new categories on the fly
- `ignore` field for assignments in ventilation spec (default: `false`)
  - Categories with `ignore: true` are excluded from the pie chart

### Changed

- Pie chart categories are now sorted by amount (descending)

## [0.2.0] - 2026-01-19

### Added

- Directory processing: parse all PDFs in a directory and output combined `out.yml`
- Parse opening and closing balance (SOLDE CREDITEUR/DEBITEUR)
- Parse TOTAL DES OPERATIONS for debit/credit totals
- Add `check_debit` and `check_credit` validation against parsed totals
- Add `SoldeType` enum (Credit/Debit) for operations and balances
- Handle operations with description on separate line
- Classify credit operations: VIR SEPA RECU, VIR CPTE A CPTE RECU, REJET RECU, RETROCESSION, REMISE CHEQUES, REMBOURST

### Changed

- Operations now use `montant` and `montant_type` instead of separate `debit`/`credit` fields
- Improved whitespace handling in amounts (including non-breaking spaces)

## [0.1.1] - 2026-01-19

### Added

- CHANGELOG.md
- GitHub Actions workflow to publish crate on release

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

[0.3.0]: https://github.com/laurentcarrie/bnp/releases/tag/v0.3.0
[0.2.0]: https://github.com/laurentcarrie/bnp/releases/tag/v0.2.0
[0.1.1]: https://github.com/laurentcarrie/bnp/releases/tag/v0.1.1
[0.1.0]: https://github.com/laurentcarrie/bnp/releases/tag/v0.1.0
