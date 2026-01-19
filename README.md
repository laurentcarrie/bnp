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
mybnp <pdf_file>
```

Parses the PDF bank statement and outputs a YAML file with the same name (e.g., `statement.pdf` → `statement.yml`).

### Example

```bash
$ mybnp statement.pdf
Parsed 42 operations (date: 2025-02-13) -> statement.yml
```

### Output Format

```yaml
date_du_releve: 2025-02-13
operations:
- date: 2025-01-16
  nature_des_operations: PRLV SEPA ...
  valeur: 2025-01-16
  debit: 100.00
  credit: null
- date: 2025-01-29
  nature_des_operations: VIR SEPA RECU ...
  valeur: 2025-01-29
  debit: null
  credit: 1000.00
```

## Library Usage

```rust
use mybnp::parse_pdf;

let releve = parse_pdf("statement.pdf")?;
println!("Date: {}", releve.date_du_releve);
for op in releve.operations {
    println!("{}: {:?} / {:?}", op.date, op.debit, op.credit);
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
