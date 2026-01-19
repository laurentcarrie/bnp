use mybnp::parse_pdf;
use std::path::Path;

fn main() {
    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("Usage: mybnp <pdf_file>");
            std::process::exit(1);
        }
    };

    match parse_pdf(&path) {
        Ok(releve) => {
            let input_path = Path::new(&path);
            let stem = input_path.file_stem().unwrap().to_str().unwrap();
            let output_path = format!("{stem}.yml");

            let yaml = serde_yaml::to_string(&releve).expect("Failed to serialize to YAML");
            std::fs::write(&output_path, &yaml).expect("Failed to write YAML file");

            println!(
                "Parsed {} operations (date: {}) -> {output_path}",
                releve.operations.len(),
                releve.date_du_releve,
            );
        }
        Err(e) => eprintln!("{e}"),
    }
}
