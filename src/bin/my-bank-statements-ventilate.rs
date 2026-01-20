use my_bank_statements::Releve;
use my_bank_statements::ventilation::model::VentilationSpec;
use my_bank_statements::ventilation::ventilate::ventilate;
use std::fs;

fn main() {
    let releves_path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("Usage: my-bank-statements-ventilate <releves.yml> <ventilation_spec.yml> [output.yml]");
            std::process::exit(1);
        }
    };

    let spec_path = match std::env::args().nth(2) {
        Some(p) => p,
        None => {
            eprintln!("Usage: my-bank-statements-ventilate <releves.yml> <ventilation_spec.yml> [output.yml]");
            std::process::exit(1);
        }
    };

    let output_path = std::env::args()
        .nth(3)
        .unwrap_or_else(|| "ventilation.yml".to_string());

    // Read releves
    let releves_content = fs::read_to_string(&releves_path).unwrap_or_else(|e| {
        eprintln!("Failed to read releves file {releves_path}: {e}");
        std::process::exit(1);
    });

    let releves: Vec<Releve> = serde_yaml::from_str(&releves_content).unwrap_or_else(|e| {
        eprintln!("Failed to parse releves: {e}");
        std::process::exit(1);
    });

    // Read ventilation spec
    let spec_content = fs::read_to_string(&spec_path).unwrap_or_else(|e| {
        eprintln!("Failed to read ventilation spec {spec_path}: {e}");
        std::process::exit(1);
    });

    let spec: VentilationSpec = serde_yaml::from_str(&spec_content).unwrap_or_else(|e| {
        eprintln!("Failed to parse ventilation spec: {e}");
        std::process::exit(1);
    });

    // Run ventilation
    match ventilate(spec, &releves) {
        Ok(result) => {
            let yaml =
                serde_yaml::to_string(&result).expect("Failed to serialize ventilation to YAML");
            fs::write(&output_path, &yaml).expect("Failed to write ventilation YAML");
            println!("Wrote ventilation to {output_path}");

            // Generate Mermaid pie chart markdown
            let mut md = format!(
                "# Ventilation: {}\n\n```mermaid\npie showData\n",
                result.spec.name
            );
            // Build a set of ignored categories
            let ignored: std::collections::HashSet<&str> = result
                .spec
                .assignments
                .iter()
                .filter(|a| a.ignore)
                .map(|a| a.name.as_str())
                .collect();

            // Collect and sort categories by amount (descending)
            let mut entries: Vec<(&String, i64)> = result
                .ventilation
                .iter()
                .filter(|(name, _)| !ignored.contains(name.as_str()))
                .map(|(name, &amount)| (name, amount))
                .collect();

            // Add "Non assigné" if there are unassigned operations
            let non_assigne = "Non assigné".to_string();
            if result.not_assigned > 0 {
                entries.push((&non_assigne, result.not_assigned));
            }

            // Sort by amount descending
            entries.sort_by(|a, b| b.1.cmp(&a.1));

            for (name, amount) in entries {
                let euros = amount as f64 / 100.0;
                md.push_str(&format!("    \"{name}\" : {euros:.2}\n"));
            }
            md.push_str("```\n");

            fs::write("ventilation.md", &md).expect("Failed to write ventilation.md");
            println!("Wrote ventilation.md");
        }
        Err(e) => {
            eprintln!("Ventilation error: {e}");
            std::process::exit(1);
        }
    }
}
