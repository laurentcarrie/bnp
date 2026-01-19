use mybnp::{Releve, parse_pdf};
use std::fs;
use std::path::Path;

fn main() {
    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("Usage: mybnp <pdf_file_or_directory>");
            std::process::exit(1);
        }
    };

    let input_path = Path::new(&path);

    if input_path.is_dir() {
        process_directory(input_path);
    } else {
        process_single_file(input_path);
    }
}

fn process_directory(dir: &Path) {
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

    let output_path = dir.join("out.yml");
    let yaml = serde_yaml::to_string(&releves).expect("Failed to serialize to YAML");
    fs::write(&output_path, &yaml).expect("Failed to write YAML file");

    println!(
        "Wrote {} releves to {}",
        releves.len(),
        output_path.display()
    );
}

fn process_single_file(input_path: &Path) {
    match parse_pdf(input_path.to_str().unwrap()) {
        Ok(releve) => {
            let stem = input_path.file_stem().unwrap().to_str().unwrap();
            let output_path = format!("{stem}.yml");

            let yaml = serde_yaml::to_string(&releve).expect("Failed to serialize to YAML");
            fs::write(&output_path, &yaml).expect("Failed to write YAML file");

            println!(
                "Parsed {} operations (date: {}) -> {output_path}",
                releve.operations.len(),
                releve.date_du_releve,
            );
        }
        Err(e) => eprintln!("{e}"),
    }
}
