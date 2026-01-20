use mybnp::{Releve, parse_pdf};
use std::fs;
use std::path::Path;

fn main() {
    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("Usage: my-bnp-parser <pdf_file_or_directory> [output.yml]");
            std::process::exit(1);
        }
    };

    let output_path = std::env::args().nth(2);
    let input_path = Path::new(&path);

    if input_path.is_dir() {
        process_directory(input_path, output_path.as_deref());
    } else {
        process_single_file(input_path, output_path.as_deref());
    }
}

fn process_directory(dir: &Path, output_path: Option<&str>) {
    let mut releves: Vec<Releve> = Vec::new();

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to read directory: {e}");
            std::process::exit(1);
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map(|e| e.to_ascii_lowercase()) == Some("pdf".into()) {
            match parse_pdf(path.to_str().unwrap()) {
                Ok(releve) => {
                    println!(
                        "Parsed {} operations (date: {}) from {}",
                        releve.operations.len(),
                        releve.date_du_releve,
                        path.file_name().unwrap().to_str().unwrap()
                    );
                    releves.push(releve);
                }
                Err(e) => eprintln!("Error parsing {}: {e}", path.display()),
            }
        }
    }

    if releves.is_empty() {
        eprintln!("No PDF files found in directory");
        std::process::exit(1);
    }

    releves.sort_by_key(|r| r.date_du_releve);

    let out = output_path
        .map(|p| Path::new(p).to_path_buf())
        .unwrap_or_else(|| dir.join("releves.yml"));

    let yaml = serde_yaml::to_string(&releves).expect("Failed to serialize to YAML");
    fs::write(&out, &yaml).expect("Failed to write YAML file");

    println!("Wrote {} releves to {}", releves.len(), out.display());
}

fn process_single_file(input_path: &Path, output_path: Option<&str>) {
    match parse_pdf(input_path.to_str().unwrap()) {
        Ok(releve) => {
            let out = output_path.map(|p| p.to_string()).unwrap_or_else(|| {
                let stem = input_path.file_stem().unwrap().to_str().unwrap();
                format!("{stem}.yml")
            });

            let yaml = serde_yaml::to_string(&releve).expect("Failed to serialize to YAML");
            fs::write(&out, &yaml).expect("Failed to write YAML file");

            println!(
                "Parsed {} operations (date: {}) -> {out}",
                releve.operations.len(),
                releve.date_du_releve,
            );
        }
        Err(e) => eprintln!("{e}"),
    }
}
