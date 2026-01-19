use chrono::NaiveDate;
use pdf_extract::extract_text;
use regex::Regex;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum SoldeType {
    Credit,
    Debit,
}

#[derive(Debug, Clone, Serialize)]
pub struct Operation {
    pub date: NaiveDate,
    pub nature_des_operations: String,
    pub valeur: NaiveDate,
    pub montant: f64,
    pub montant_type: SoldeType,
}

#[derive(Debug, Clone, Serialize)]
pub struct Solde {
    pub solde_type: SoldeType,
    pub montant: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Releve {
    pub date_du_releve: NaiveDate,
    pub solde_ouverture: Option<Solde>,
    pub solde_cloture: Option<Solde>,
    pub total_des_operations_debit: Option<f64>,
    pub total_des_operations_credit: Option<f64>,
    pub check_debit: f64,
    pub check_credit: f64,
    pub operations: Vec<Operation>,
}

fn parse_amount(s: &str) -> Option<f64> {
    if s.is_empty() {
        return None;
    }
    // Remove all whitespace (including non-breaking spaces) and replace comma with period
    let cleaned: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    let cleaned = cleaned.replace(",", ".");
    cleaned.parse::<f64>().ok()
}

fn french_month_to_number(month: &str) -> Option<u32> {
    match month.to_lowercase().as_str() {
        "janvier" => Some(1),
        "février" | "fevrier" => Some(2),
        "mars" => Some(3),
        "avril" => Some(4),
        "mai" => Some(5),
        "juin" => Some(6),
        "juillet" => Some(7),
        "août" | "aout" => Some(8),
        "septembre" => Some(9),
        "octobre" => Some(10),
        "novembre" => Some(11),
        "décembre" | "decembre" => Some(12),
        _ => None,
    }
}

struct ReleveDateInfo {
    day: u32,
    month: u32,
    year: i32,
}

fn parse_date_du_releve(text: &str) -> Option<ReleveDateInfo> {
    let re = Regex::new(r"du \d+ \w+ \d+ au (\d+) (\w+) (\d+)").unwrap();

    if let Some(caps) = re.captures(text) {
        let day: u32 = caps.get(1)?.as_str().parse().ok()?;
        let month_str = caps.get(2)?.as_str();
        let year: i32 = caps.get(3)?.as_str().parse().ok()?;
        let month = french_month_to_number(month_str)?;

        return Some(ReleveDateInfo { day, month, year });
    }
    None
}

pub fn compute_year(op_month: u32, releve_month: u32, releve_year: i32) -> i32 {
    // If operation month is much greater than releve month (e.g., Dec vs Jan/Feb),
    // the operation is from the previous year
    if op_month > releve_month && (op_month - releve_month) > 6 {
        releve_year - 1
    } else {
        releve_year
    }
}

fn parse_date_with_year(date_str: &str, releve: &ReleveDateInfo) -> Option<NaiveDate> {
    // date_str is "DD.MM"
    let parts: Vec<&str> = date_str.split('.').collect();
    if parts.len() == 2 {
        let day: u32 = parts[0].parse().ok()?;
        let month: u32 = parts[1].parse().ok()?;
        let year = compute_year(month, releve.month, releve.year);
        NaiveDate::from_ymd_opt(year, month, day)
    } else {
        None
    }
}

fn is_stop_line(line: &str, line_re: &Regex, line_re_no_text: &Regex) -> bool {
    line_re.is_match(line)
        || line_re_no_text.is_match(line)
        || line.contains("BNP PARIBAS")
        || line.starts_with("P.")
        || line.starts_with("504")
        || line.starts_with("SCPT")
        || line.contains("RELEVE DE COMPTE")
        || line.contains("D ate")
        || line.contains("GARENNE COL")
        || line.contains("RIB :")
        || line.contains("M LAURENT")
        || line.contains("APPARTEMENT")
        || line.contains("TOTAL DES OPERATIONS")
        || line.contains("SOLDE CREDITEUR")
        || line.contains("SOLDE DEBITEUR")
}

fn parse_soldes(text: &str) -> (Option<Solde>, Option<Solde>) {
    let re =
        Regex::new(r"SOLDE (CREDITEUR|DEBITEUR) AU \d{2}\.\d{2}\.\d{4}\s+([\d\s]+,\d{2})").unwrap();

    let matches: Vec<_> = re.captures_iter(text).collect();

    let ouverture = matches.first().and_then(|caps| {
        let solde_type = if caps.get(1)?.as_str() == "CREDITEUR" {
            SoldeType::Credit
        } else {
            SoldeType::Debit
        };
        let montant = parse_amount(caps.get(2)?.as_str())?;
        Some(Solde {
            solde_type,
            montant,
        })
    });

    let cloture = matches.last().and_then(|caps| {
        let solde_type = if caps.get(1)?.as_str() == "CREDITEUR" {
            SoldeType::Credit
        } else {
            SoldeType::Debit
        };
        let montant = parse_amount(caps.get(2)?.as_str())?;
        Some(Solde {
            solde_type,
            montant,
        })
    });

    (ouverture, cloture)
}

fn parse_total_des_operations(text: &str) -> (Option<f64>, Option<f64>) {
    let re = Regex::new(r"TOTAL DES OPERATIONS\s+([\d\s]+,\d{2})\s+([\d\s]+,\d{2})").unwrap();

    if let Some(caps) = re.captures(text) {
        let debit = caps.get(1).and_then(|m| parse_amount(m.as_str()));
        let credit = caps.get(2).and_then(|m| parse_amount(m.as_str()));
        return (debit, credit);
    }
    (None, None)
}

fn parse_operations(text: &str, releve: &ReleveDateInfo) -> Vec<Operation> {
    let mut operations = Vec::new();

    // Pattern with text on same line: "03.01 03.01 1,50* COMMISSIONS..."
    let line_re =
        Regex::new(r"^(\d{2}\.\d{2})\s+(\d{2}\.\d{2})\s+([\d\s]+,\d{2})([A-Z*].*)$").unwrap();
    // Pattern with text on next line: "03.01 03.01 1,50"
    let line_re_no_text =
        Regex::new(r"^(\d{2}\.\d{2})\s+(\d{2}\.\d{2})\s+([\d\s]+,\d{2})$").unwrap();

    let lines: Vec<&str> = text.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        // Try pattern with text on same line first
        let (date_raw, valeur_raw, amount_str, mut nature) =
            if let Some(caps) = line_re.captures(line) {
                (
                    caps.get(1).unwrap().as_str(),
                    caps.get(2).unwrap().as_str(),
                    caps.get(3).unwrap().as_str().trim(),
                    caps.get(4).unwrap().as_str().trim().to_string(),
                )
            } else if let Some(caps) = line_re_no_text.captures(line) {
                // Text starts on next line
                (
                    caps.get(1).unwrap().as_str(),
                    caps.get(2).unwrap().as_str(),
                    caps.get(3).unwrap().as_str().trim(),
                    String::new(),
                )
            } else {
                i += 1;
                continue;
            };

        let date = match parse_date_with_year(date_raw, releve) {
            Some(d) => d,
            None => {
                i += 1;
                continue;
            }
        };
        let valeur = match parse_date_with_year(valeur_raw, releve) {
            Some(d) => d,
            None => {
                i += 1;
                continue;
            }
        };

        i += 1;
        while i < lines.len() {
            let next_line = lines[i].trim();

            if next_line.is_empty() {
                i += 1;
                continue;
            }

            if is_stop_line(next_line, &line_re, &line_re_no_text) {
                break;
            }

            if !nature.is_empty() {
                nature.push(' ');
            }
            nature.push_str(next_line);
            i += 1;
        }

        let montant = parse_amount(amount_str).unwrap_or(0.0);

        let montant_type = if nature.contains("VIR SEPA RECU")
            || nature.contains("VIR CPTE A CPTE RECU")
            || nature.contains("REJET RECU")
            || nature.contains("RETROCESSION")
            || nature.contains("REMISE CHEQUES")
            || nature.contains("REMBOURST")
        {
            SoldeType::Credit
        } else {
            SoldeType::Debit
        };

        operations.push(Operation {
            date,
            nature_des_operations: nature.trim().to_string(),
            valeur,
            montant,
            montant_type,
        });

        continue;
    }

    operations
}

pub fn parse_pdf(path: &str) -> Result<Releve, String> {
    let text = extract_text(path).map_err(|e| format!("Error extracting text: {e}"))?;

    let releve_info = parse_date_du_releve(&text).ok_or("Could not parse date du releve")?;

    let date_du_releve =
        NaiveDate::from_ymd_opt(releve_info.year, releve_info.month, releve_info.day)
            .ok_or("Invalid date du releve")?;

    let (solde_ouverture, solde_cloture) = parse_soldes(&text);
    let (total_des_operations_debit, total_des_operations_credit) =
        parse_total_des_operations(&text);
    let operations = parse_operations(&text, &releve_info);

    let check_debit: f64 = (operations
        .iter()
        .filter(|op| matches!(op.montant_type, SoldeType::Debit))
        .map(|op| op.montant)
        .sum::<f64>()
        * 100.0)
        .round()
        / 100.0;

    let check_credit: f64 = (operations
        .iter()
        .filter(|op| matches!(op.montant_type, SoldeType::Credit))
        .map(|op| op.montant)
        .sum::<f64>()
        * 100.0)
        .round()
        / 100.0;

    if let Some(total_debit) = total_des_operations_debit
        && (total_debit - check_debit).abs() > 0.01
    {
        return Err(format!(
            "Debit mismatch: total_des_operations_debit={total_debit} but check_debit={check_debit}"
        ));
    }

    if let Some(total_credit) = total_des_operations_credit
        && (total_credit - check_credit).abs() > 0.01
    {
        return Err(format!(
            "Credit mismatch: total_des_operations_credit={total_credit} but check_credit={check_credit}"
        ));
    }

    Ok(Releve {
        date_du_releve,
        solde_ouverture,
        solde_cloture,
        total_des_operations_debit,
        total_des_operations_credit,
        check_debit,
        check_credit,
        operations,
    })
}
