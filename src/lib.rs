use chrono::NaiveDate;
use pdf_extract::extract_text;
use regex::Regex;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Operation {
    pub date: NaiveDate,
    pub nature_des_operations: String,
    pub valeur: NaiveDate,
    pub debit: Option<f64>,
    pub credit: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Releve {
    pub date_du_releve: NaiveDate,
    pub operations: Vec<Operation>,
}

fn parse_amount(s: &str) -> Option<f64> {
    if s.is_empty() {
        return None;
    }
    let cleaned: String = s.replace(" ", "").replace(",", ".");
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

fn is_stop_line(line: &str, line_re: &Regex) -> bool {
    line_re.is_match(line)
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
}

fn parse_operations(text: &str, releve: &ReleveDateInfo) -> Vec<Operation> {
    let mut operations = Vec::new();

    let line_re =
        Regex::new(r"^(\d{2}\.\d{2})\s+(\d{2}\.\d{2})\s+([\d\s]+,\d{2})([A-Z*].*)$").unwrap();

    let lines: Vec<&str> = text.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        if let Some(caps) = line_re.captures(line) {
            let date_raw = caps.get(1).unwrap().as_str();
            let valeur_raw = caps.get(2).unwrap().as_str();
            let amount_str = caps.get(3).unwrap().as_str().trim();
            let mut nature = caps.get(4).unwrap().as_str().trim().to_string();

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

                if is_stop_line(next_line, &line_re) {
                    break;
                }

                nature.push(' ');
                nature.push_str(next_line);
                i += 1;
            }

            let amount = parse_amount(amount_str);

            let is_credit = nature.contains("VIR SEPA RECU")
                || nature.contains("REJET RECU")
                || nature.contains("RETROCESSION");

            let (debit, credit) = if is_credit {
                (None, amount)
            } else {
                (amount, None)
            };

            operations.push(Operation {
                date,
                nature_des_operations: nature.trim().to_string(),
                valeur,
                debit,
                credit,
            });

            continue;
        }

        i += 1;
    }

    operations
}

pub fn parse_pdf(path: &str) -> Result<Releve, String> {
    let text = extract_text(path).map_err(|e| format!("Error extracting text: {e}"))?;

    let releve_info = parse_date_du_releve(&text).ok_or("Could not parse date du releve")?;

    let date_du_releve =
        NaiveDate::from_ymd_opt(releve_info.year, releve_info.month, releve_info.day)
            .ok_or("Invalid date du releve")?;

    let operations = parse_operations(&text, &releve_info);

    Ok(Releve {
        date_du_releve,
        operations,
    })
}
