# mybnp

[![CI](https://github.com/laurentcarrie/bnp/actions/workflows/ci.yml/badge.svg)](https://github.com/laurentcarrie/bnp/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/mybnp.svg)](https://crates.io/crates/mybnp)

A Rust library and CLI tool to parse BNP Paribas bank statements (PDF) and extract operations to YAML.

## Installation

From crates.io:

```bash
cargo install mybnp
```

From source:

```bash
cargo install --path .
```

## CLI Usage

```bash
mybnp <pdf_file_or_directory>
```

- **Single file**: Parses the PDF and outputs `<filename>.yml`
- **Directory**: Parses all PDFs in the directory and outputs `out.yml` with all releves sorted by date

### Examples

```bash
$ mybnp statement.pdf
Parsed 42 operations (date: 2025-02-13) -> statement.yml

$ mybnp pdfs/
Parsed 104 operations (date: 2024-01-13) from statement1.pdf
Parsed 89 operations (date: 2024-02-13) from statement2.pdf
Wrote 2 releves to pdfs/out.yml
```

### Output Format

```yaml
- date_du_releve: 2025-02-13
  solde_ouverture:
    solde_type: Credit
    montant: 1500.00
  solde_cloture:
    solde_type: Credit
    montant: 2000.00
  total_des_operations_debit: 3500.00
  total_des_operations_credit: 4000.00
  check_debit: 3500.00
  check_credit: 4000.00
  operations:
  - date: 2025-01-16
    nature_des_operations: PRLV SEPA ...
    valeur: 2025-01-16
    montant: 100.00
    montant_type: Debit
  - date: 2025-01-29
    nature_des_operations: VIR SEPA RECU ...
    valeur: 2025-01-29
    montant: 1000.00
    montant_type: Credit
```

The tool validates that `check_debit` equals `total_des_operations_debit` and `check_credit` equals `total_des_operations_credit`. If there's a mismatch, an error is reported.

## Library Usage

```rust
use mybnp::{parse_pdf, SoldeType};

let releve = parse_pdf("statement.pdf")?;
println!("Date: {}", releve.date_du_releve);
for op in releve.operations {
    let sign = match op.montant_type {
        SoldeType::Credit => "+",
        SoldeType::Debit => "-",
    };
    println!("{}: {}{}", op.date, sign, op.montant);
}
```

## Build

```bash
cargo build --release
```

## Tests

```bash
cargo test
```
