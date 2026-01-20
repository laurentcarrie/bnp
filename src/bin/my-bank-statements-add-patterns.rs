use my_bank_statements::Releve;
use my_bank_statements::ventilation::model::{Assignment, VentilationSpec};
use my_bank_statements::ventilation::ventilate::ventilate;
use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};

/// Keywords that suggest a category
struct KeywordSuggestion {
    keywords: Vec<&'static str>,
    category: &'static str,
    pattern_suggestion: Option<&'static str>,
}

fn get_suggestions() -> Vec<KeywordSuggestion> {
    vec![
        // Restaurants / Bars
        KeywordSuggestion {
            keywords: vec![
                "RESTAURANT",
                "REST ",
                "RESTO",
                "BRASSERIE",
                "BISTRO",
                "CAFE ",
                "PIZZA",
                "SUSHI",
                "KEBAB",
                "BURGER",
                "MCDONALD",
                "KFC",
                "QUICK",
                "SUBWAY",
            ],
            category: "Restaurants",
            pattern_suggestion: None,
        },
        KeywordSuggestion {
            keywords: vec!["BAR ", "PUB ", "TAVERN", "BIERE", "BEER"],
            category: "Restaurants",
            pattern_suggestion: None,
        },
        // Supermarkets
        KeywordSuggestion {
            keywords: vec![
                "CARREFOUR",
                "MONOPRIX",
                "MONOP",
                "FRANPRIX",
                "AUCHAN",
                "LECLERC",
                "LIDL",
                "INTERMARCHE",
                "CASINO",
                "SUPER U",
                "SPAR",
                "PICARD",
                "ALDI",
                "COSTCO",
                "PRIMEUR",
                "MARCHE",
                "BOUCHERIE",
                "FROMAGERIE",
                "POISSONNERIE",
                "BOULANG",
                "PATISSERIE",
                "EPICERIE",
            ],
            category: "Supermarches",
            pattern_suggestion: None,
        },
        // Transport
        KeywordSuggestion {
            keywords: vec![
                "RATP",
                "SNCF",
                "SNCB",
                "UBER",
                "BOLT",
                "TAXI",
                "NAVIGO",
                "METRO",
                "PARKING",
                "INDIGO",
                "EFFIA",
                "STATIONNEMENT",
                "PEAGE",
                "AUTOROUTE",
                "APRR",
                "SANEF",
                "ASF",
                "COFIROUTE",
                "ESCOTA",
                "TOTAL",
                "ESSO",
                "BP ",
                "SHELL",
                "STATION",
                "ESSENCE",
                "GARAGE",
            ],
            category: "Transport",
            pattern_suggestion: None,
        },
        // Leisure
        KeywordSuggestion {
            keywords: vec![
                "CIRQUE",
                "CINEMA",
                "UGC",
                "PATHE",
                "THEATRE",
                "CONCERT",
                "MUSEE",
                "MUSEUM",
                "SPECTACLE",
                "FNAC",
                "CULTURA",
                "CONSERVATOIRE",
                "BILLETREDUC",
                "TICKET",
                "WEEZEVENT",
                "DICE.FM",
                "HELLOASSO",
            ],
            category: "Loisirs",
            pattern_suggestion: None,
        },
        // Health
        KeywordSuggestion {
            keywords: vec![
                "PHARMACIE",
                "PHARMA",
                "MEDECIN",
                "DOCTEUR",
                "DR ",
                "LABORATOIRE",
                "DENTAIRE",
                "OPTICIEN",
                "HENNER",
                "MUTUELLE",
                "SANTE",
            ],
            category: "Sante",
            pattern_suggestion: None,
        },
        // Subscriptions
        KeywordSuggestion {
            keywords: vec![
                "NETFLIX",
                "SPOTIFY",
                "AMAZON PRIME",
                "CANAL",
                "ORANGE",
                "SFR",
                "BOUYGUES",
                "FREE MOBILE",
                "DEEZER",
                "DISNEY",
            ],
            category: "Abonnements",
            pattern_suggestion: None,
        },
        // Sport
        KeywordSuggestion {
            keywords: vec![
                "FITNESS",
                "GYM",
                "SPORT",
                "DECATHLON",
                "INTERSPORT",
                "GO SPORT",
            ],
            category: "Sport",
            pattern_suggestion: None,
        },
        // Housing
        KeywordSuggestion {
            keywords: vec![
                "LOYER",
                "EDF",
                "ENGIE",
                "GAZ",
                "ELECTRICITE",
                "SYNDIC",
                "IMMOBILIER",
                "AGENCE IMMO",
            ],
            category: "Loyers",
            pattern_suggestion: None,
        },
        // Cloud
        KeywordSuggestion {
            keywords: vec![
                "KAMATERA",
                "AWS",
                "GOOGLE CLOUD",
                "AZURE",
                "DIGITALOCEAN",
                "OVH",
                "SCALEWAY",
                "ONLINE SAS",
                "DEDIBOX",
            ],
            category: "Cloud",
            pattern_suggestion: None,
        },
        // Going out / Nightlife
        KeywordSuggestion {
            keywords: vec![
                "SUPERSONIC",
                "CLUB",
                "DISCOTHEQUE",
                "CONCERT",
                "LIVE",
                "MUSIC",
            ],
            category: "Sortie",
            pattern_suggestion: None,
        },
        // Cash withdrawal
        KeywordSuggestion {
            keywords: vec!["RETRAIT DAB", "DAB ", "DISTRIBUTEUR"],
            category: "DAB",
            pattern_suggestion: None,
        },
        // Investments
        KeywordSuggestion {
            keywords: vec!["CARDIF", "ASSURANCE VIE", "PLACEMENT", "EPARGNE"],
            category: "Placements",
            pattern_suggestion: None,
        },
        // Equipment
        KeywordSuggestion {
            keywords: vec![
                "DARTY",
                "BOULANGER",
                "LDLC",
                "MATERIEL.NET",
                "IKEA",
                "LEROY MERLIN",
                "CASTORAMA",
                "BRICORAMA",
                "BRICOMAN",
                "BRICOMARCHE",
                "AMAZON",
                "CDISCOUNT",
            ],
            category: "Equipement",
            pattern_suggestion: None,
        },
        // Taxes
        KeywordSuggestion {
            keywords: vec!["DGFIP", "IMPOT", "TRESOR PUBLIC", "AMENDE"],
            category: "Impots",
            pattern_suggestion: None,
        },
        // Insurance
        KeywordSuggestion {
            keywords: vec![
                "MAIF", "MACIF", "MAAF", "AXA", "ALLIANZ", "GROUPAMA", "MATMUT", "GMF",
            ],
            category: "Assurance",
            pattern_suggestion: None,
        },
        // Transfers (often personal)
        KeywordSuggestion {
            keywords: vec!["VIREMENT SEPA EMIS", "VIR CPTE A CPTE", "VIRT CPTE A CPTE"],
            category: "Virements",
            pattern_suggestion: None,
        },
        // Checks
        KeywordSuggestion {
            keywords: vec!["CHEQUE"],
            category: "Cheques",
            pattern_suggestion: None,
        },
        // Online payments
        KeywordSuggestion {
            keywords: vec!["PAYPAL", "STRIPE", "SUMUP", "ZETTLE", "LYDIA"],
            category: "Paiements",
            pattern_suggestion: None,
        },
        // Travel / Hotels
        KeywordSuggestion {
            keywords: vec![
                "AIRBNB",
                "BOOKING",
                "HOTEL",
                "BKG*HOTEL",
                "AIR FRANCE",
                "EASYJET",
                "RYANAIR",
                "VUELING",
            ],
            category: "Voyages",
            pattern_suggestion: None,
        },
        // Clothing
        KeywordSuggestion {
            keywords: vec![
                "ZARA",
                "H&M",
                "UNIQLO",
                "KIABI",
                "ARMAND THIERY",
                "CELIO",
                "JULES",
                "CAMAIEU",
            ],
            category: "Vetements",
            pattern_suggestion: None,
        },
    ]
}

fn suggest_category(
    operation: &str,
    suggestions: &[KeywordSuggestion],
    existing_categories: &[String],
) -> Option<(String, Option<String>)> {
    let op_upper = operation.to_uppercase();

    for suggestion in suggestions {
        for keyword in &suggestion.keywords {
            if op_upper.contains(keyword) {
                // Check if the suggested category exists
                let cat = suggestion.category.to_string();
                if existing_categories.iter().any(|c| c == &cat) {
                    return Some((cat, suggestion.pattern_suggestion.map(|s| s.to_string())));
                } else {
                    // Category doesn't exist, suggest creating it
                    return Some((
                        format!("NEW:{cat}"),
                        suggestion.pattern_suggestion.map(|s| s.to_string()),
                    ));
                }
            }
        }
    }
    None
}

fn extract_pattern_from_operation(operation: &str) -> String {
    // Try to extract a meaningful pattern from the operation
    // Remove common prefixes like dates, card numbers, etc.
    let op = operation.to_uppercase();

    // Remove "DU DDMMYY" prefix
    let op = regex::Regex::new(r"^DU \d{6} ")
        .unwrap()
        .replace(&op, "")
        .to_string();

    // Remove "FACTURE(S) CARTE ... DU DDMMYY" prefix
    let op = regex::Regex::new(r"^FACTURE\(S\) CARTE \S+ DU \d{6} ")
        .unwrap()
        .replace(&op, "")
        .to_string();

    // Take the first meaningful words (up to 3)
    let words: Vec<&str> = op.split_whitespace().take(3).collect();
    if words.is_empty() {
        return regex::escape(operation);
    }

    // Create a pattern from the first words
    words.join(" ")
}

fn main() {
    let releves_path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!(
                "Usage: my-bank-statements-add-patterns <releves.yml> <ventilation_spec.yml> [output.yml]"
            );
            std::process::exit(1);
        }
    };

    let spec_path = match std::env::args().nth(2) {
        Some(p) => p,
        None => {
            eprintln!(
                "Usage: my-bank-statements-add-patterns <releves.yml> <ventilation_spec.yml> [output.yml]"
            );
            std::process::exit(1);
        }
    };

    let output_path = std::env::args()
        .nth(3)
        .unwrap_or_else(|| "ventilation_spec_updated.yml".to_string());

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

    let mut spec: VentilationSpec = serde_yaml::from_str(&spec_content).unwrap_or_else(|e| {
        eprintln!("Failed to parse ventilation spec: {e}");
        std::process::exit(1);
    });

    // Run ventilation
    let result = match ventilate(spec.clone(), &releves) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Ventilation error: {e}");
            std::process::exit(1);
        }
    };

    if result.not_assigned_operations.is_empty() {
        println!("No unassigned operations found!");
        return;
    }

    println!(
        "Found {} unassigned operations.\n",
        result.not_assigned_operations.len()
    );

    // Collect unique operation descriptions
    let mut seen: HashSet<String> = HashSet::new();
    let unique_operations: Vec<_> = result
        .not_assigned_operations
        .iter()
        .filter(|op| seen.insert(op.nature_des_operations.clone()))
        .collect();

    println!(
        "Processing {} unique operation descriptions...\n",
        unique_operations.len()
    );

    // Get suggestions
    let suggestions = get_suggestions();

    let mut changes_made = false;
    let mut auto_mode = false;
    let mut skip_all = false;

    for op in unique_operations {
        if skip_all {
            break;
        }

        // Get category names (refresh each iteration in case we added new ones)
        let category_names: Vec<String> = spec.assignments.iter().map(|a| a.name.clone()).collect();

        // Try to suggest a category
        let suggestion = suggest_category(&op.nature_des_operations, &suggestions, &category_names);
        let suggested_pattern = extract_pattern_from_operation(&op.nature_des_operations);

        println!("----------------------------------------");
        println!("Operation: {}", op.nature_des_operations);
        println!("Amount: {:.2} EUR", op.montant as f64 / 100.0);
        println!("Date: {}", op.date);

        if let Some((ref cat, _)) = suggestion {
            if let Some(new_cat) = cat.strip_prefix("NEW:") {
                println!("\n>>> Suggested: Create new category '{new_cat}'");
            } else {
                println!("\n>>> Suggested: {cat} (pattern: {suggested_pattern})");
            }
        }

        println!();
        println!("Actions:");
        println!("  [Enter] Accept suggestion (if any)");
        println!("  0. Skip");
        println!("  a. Auto-accept all suggestions");
        println!("  q. Quit (save changes)");
        for (i, name) in category_names.iter().enumerate() {
            println!("  {}. {}", i + 1, name);
        }
        println!("  n. Create new category");
        println!();

        print!("Choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if !auto_mode {
            io::stdin().read_line(&mut input).unwrap();
        }
        let input = input.trim();

        // Handle auto mode
        if input == "a" || input == "A" {
            auto_mode = true;
            println!("Auto-accept mode enabled.\n");
        }

        // Handle quit
        if input == "q" || input == "Q" {
            skip_all = true;
            continue;
        }

        // Handle skip
        if input == "0" {
            println!("Skipped.\n");
            continue;
        }

        // Handle Enter (accept suggestion) or auto mode
        if input.is_empty() || auto_mode {
            if let Some((cat, _)) = suggestion {
                if let Some(new_cat) = cat.strip_prefix("NEW:") {
                    let new_name = new_cat.to_string();
                    spec.assignments.push(Assignment {
                        name: new_name.clone(),
                        patterns: vec![suggested_pattern.clone()],
                        ignore: false,
                    });
                    println!("Created category '{new_name}' with pattern '{suggested_pattern}'.\n");
                    changes_made = true;
                } else {
                    // Find the category and add the pattern
                    if let Some(assignment) = spec.assignments.iter_mut().find(|a| a.name == cat) {
                        if !assignment.patterns.contains(&suggested_pattern) {
                            assignment.patterns.push(suggested_pattern.clone());
                            println!("Added pattern '{suggested_pattern}' to category '{cat}'.\n");
                            changes_made = true;
                        } else {
                            println!("Pattern already exists in category '{cat}'.\n");
                        }
                    }
                }
                continue;
            } else if auto_mode {
                // No suggestion in auto mode, skip
                continue;
            } else {
                println!("No suggestion available. Please choose a category.\n");
                continue;
            }
        }

        // Handle new category
        if input == "n" || input == "N" {
            print!("New category name: ");
            io::stdout().flush().unwrap();

            let mut new_name = String::new();
            io::stdin().read_line(&mut new_name).unwrap();
            let new_name = new_name.trim().to_string();

            if new_name.is_empty() {
                println!("Empty name, skipped.\n");
                continue;
            }

            print!("Pattern to add (default: {suggested_pattern}): ");
            io::stdout().flush().unwrap();

            let mut pattern = String::new();
            io::stdin().read_line(&mut pattern).unwrap();
            let pattern = pattern.trim();

            let pattern = if pattern.is_empty() {
                suggested_pattern.clone()
            } else {
                pattern.to_string()
            };

            spec.assignments.push(Assignment {
                name: new_name.clone(),
                patterns: vec![pattern.clone()],
                ignore: false,
            });

            println!("Created category '{new_name}' with pattern '{pattern}'.\n");
            changes_made = true;
            continue;
        }

        // Handle number choice
        if let Ok(choice) = input.parse::<usize>() {
            if choice > 0 && choice <= category_names.len() {
                let category_idx = choice - 1;
                let category_name = &category_names[category_idx];

                print!("Pattern to add (default: {suggested_pattern}): ");
                io::stdout().flush().unwrap();

                let mut pattern = String::new();
                io::stdin().read_line(&mut pattern).unwrap();
                let pattern = pattern.trim();

                let pattern = if pattern.is_empty() {
                    suggested_pattern.clone()
                } else {
                    pattern.to_string()
                };

                spec.assignments[category_idx]
                    .patterns
                    .push(pattern.clone());

                println!("Added pattern '{pattern}' to category '{category_name}'.\n");
                changes_made = true;
            } else {
                println!("Invalid choice, skipped.\n");
            }
        } else {
            println!("Invalid input, skipped.\n");
        }
    }

    if changes_made {
        // Write updated spec
        let yaml = serde_yaml::to_string(&spec).expect("Failed to serialize spec to YAML");
        fs::write(&output_path, &yaml).expect("Failed to write updated spec");
        println!("\nWrote updated spec to {output_path}");
    } else {
        println!("\nNo changes made.");
    }
}
