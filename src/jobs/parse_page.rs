use chrono::{Datelike, NaiveDate};
use regex::Regex;
use rocket::serde::{Deserialize, Serialize};

use crate::jobs::enclosing_dates::enclosing_dates_of_page;
use crate::jobs::model::{Row, Table, TotalDesOperations, Value};
use crate::jobs::solde::get_xml_solde;
use crate::jobs::total_des_operations::total_des_operations_of_page;
use crate::jobs::xml_model::{Item, Page, Text};
use crate::util::error::MyError;

fn guess_poste(nature: String) -> String {
    if nature.contains("/MOTIF SALAIRE") {
        return "salaire".to_string();
    }
    if nature.starts_with("VIR SEPA RECU /DE CPAM") {
        return "secu".to_string();
    };
    if nature.starts_with("PRLV SEPA DGFIP IMPOT") {
        return "taxes".to_string();
    };

    if nature.contains("DU 041223 ATRIUM GARENNATRIUM/") {
        return "alimentation".to_string();
    };

    if nature.starts_with("PRLV SEPA HENNER GMC-HENNER-GMC") {
        return "mutuelle".to_string();
    };

    if nature.contains("EMIS /MOTIF") && nature.contains("IMMO") {
        return "emprunt_immo".to_string();
    };

    if nature.starts_with("PRLV SEPA NAVIGO ANNUEL") {
        return "navigo".to_string();
    };

    if nature.starts_with("PRLV SEPA CARDIF ASSURANCE VIE") {
        return "assurance_vie".to_string();
    };

    if nature.starts_with("PRLV SEPA MAIF") {
        return "maif".to_string();
    };

    if nature.starts_with("VIREMENT SEPA EMIS") && nature.contains("TAMARA") {
        return "virement_tam".to_string();
    };

    if nature.starts_with("RETRAIT DAB") {
        return "retrait_dab".to_string();
    };

    if nature.contains("GATFIC") {
        return "gatfic".to_string();
    };

    if nature.contains("LEROY MERLIN") {
        return "maison".to_string();
    };

    if nature.contains("LOYER PARKING") {
        return "box".to_string();
    };

    if nature.contains("EDF") {
        return "edf".to_string();
    };

    if nature.contains("ORANGE") {
        return "orange".to_string();
    };

    if nature.contains("dedibox") {
        return "orange".to_string();
    };

    if nature.starts_with("PRLV SEPA CONSERVATOIRE MUSIQUE") {
        return "conservatoire".to_string();
    };

    "?".to_string()
}

fn get_nature(
    texts: &Vec<&Text>,
    date: &Text,
    left_of_nature: u32,
    left_of_valeur: u32,
    total_des_operations: &Option<TotalDesOperations>,
) -> Result<String, MyError> {
    let next_date = texts
        .iter()
        .filter(|t| t.top > date.top && t.left == date.left)
        .filter(|t| match total_des_operations {
            None => true,
            Some(total) => t.top < total.top,
        })
        .next();
    let last_line = match (next_date, total_des_operations) {
        (Some(d), _) => d.top - 1,
        (None, Some(total)) => total.top - 1,
        (None, None) => 10000,
    };
    // let next_date = next_date.ok_or(MyError::Message("could not find next date".to_string()))? ;
    let candidates = texts
        .iter()
        .filter(|t| {
            t.top >= date.top - 1
                && t.left >= left_of_nature
                && (t.left + t.width) < left_of_valeur
                && t.top < last_line
        })
        .collect::<Vec<_>>();
    // dbg!(candidates);
    let natures = candidates
        .iter()
        .map(|t| t.value.clone())
        .collect::<Vec<_>>();
    let nature = natures.join(" ");
    if nature.contains("GATFIC") {
        dbg!(&nature);
        dbg!(&total_des_operations);
        dbg!(&last_line);
        dbg!(&next_date);
    }
    Ok(nature)
}

fn validate_candidate(texts: &Vec<&Text>, candidate: &Text, check: &str) -> bool {
    let others = texts
        .iter()
        .filter(|t| (t.top as i32 - candidate.top as i32).abs() < 3)
        .filter(|t| *t != &candidate)
        .filter(|t| t.value == check)
        .next();
    // dbg!(&candidate);
    // dbg!(&check);
    // dbg!(&others);
    others.is_some()
}
fn find_column(texts: &Vec<&Text>, what: &str, check: &str) -> Result<Text, MyError> {
    // in the xml file, the "Date" field of the pdf document is in two parts, "D" and "ate"
    let candidate = texts
        .iter()
        .filter(|t| t.value == what)
        .filter(|t| validate_candidate(texts, t, check))
        .collect::<Vec<_>>();
    // dbg!(&candidate);
    // let candidate = candidate.get(0).unwrap() ;

    let candidate = candidate
        .get(0)
        .ok_or(MyError::Message(format!(
            "could not find {} ; {}",
            what, check
        )))?
        .clone()
        .clone()
        .clone();
    // dbg!(&candidate);
    Ok(candidate)
}

fn row_of_date(
    date: Text,
    texts: &Vec<&Text>,
    (ec1, ec2): (NaiveDate, NaiveDate),
    left_of_nature: u32,
    left_of_valeur: u32,
    right_of_credit: u32,
    right_of_debit: u32,
    total_des_operations: &Option<TotalDesOperations>,
) -> Result<Row, MyError> {
    let mut candidates = texts
        .iter()
        .filter(|t| (t.top as i32 - date.top as i32).abs() < 2)
        .collect::<Vec<_>>();
    candidates.sort_by(|a, b| a.left.cmp(&b.left));
    // dbg!(&candidates);
    let naivedate = &candidates
        .get(0)
        .ok_or(MyError::Message("could not find 0".to_string()))?
        .value;
    let re = Regex::new(r"(\d\d)\.(\d\d)").unwrap();
    let caps = re
        .captures(naivedate.as_str())
        .ok_or(MyError::Message(format!("bad date : {}", naivedate)))?;
    let day = caps
        .get(1)
        .ok_or(MyError::Message("could not find day".to_string()))?
        .as_str()
        .parse::<u32>()?;
    let month = caps
        .get(2)
        .ok_or(MyError::Message("could not find month".to_string()))?
        .as_str()
        .parse::<u32>()?;
    let year = if month == ec1.month() {
        ec1.year()
    } else if month == ec2.month() {
        ec1.year()
    } else {
        return Err(MyError::Message(format!(
            "{} does not match {} or {}",
            month, ec1, ec2
        )));
    };
    let naivedate =
        NaiveDate::parse_from_str(format!("{}.{}.{}", day, month, year).as_str(), "%d.%m.%Y")?;

    // rows has n elements, nature consumes nn elements
    // 0 : date, 1..1+nn : nature, 2+nn : valeur, 3+nn: value
    // nn = candidate.len - 3
    let nn = candidates.len() - 3;

    // let nature = &candidates
    //     .get(1)
    //     .ok_or(MyError::Message("could not find 1".to_string()))?
    //     .value;
    let nature = get_nature(
        texts,
        &date,
        left_of_nature,
        left_of_valeur,
        total_des_operations,
    )?;
    let value_text = &candidates
        .get(candidates.len() - 1)
        .ok_or(MyError::Message("could not find 3".to_string()))?;
    let value = value_text
        .value
        .replace(" ", "")
        .replace(",", "")
        .parse::<u32>()?;
    let value = if value_text.left > right_of_debit {
        Value::Credit(value)
    } else {
        Value::Debit(value)
        //         return Err(MyError::Message(format!("cannot determine debit or credit, left={}, width={}, right_of_credit={}, right_of_debit={}, nature={}",
        // value_text.left,value_text.width,right_of_credit,right_of_debit,&nature
        //         )));
    };
    let poste = guess_poste(nature.clone());
    let row = Row {
        date: naivedate,
        nature: nature.clone(),
        value: value,
        poste: poste.clone(),
        commentaire: "".to_string(),
    };
    Ok(row)
}

pub fn parse_page(page: &Page, releve: NaiveDate) -> Result<Table, MyError> {
    // println!("parse page #{}", page.number);
    let ec = enclosing_dates_of_page(page)?;
    // let solde = get_xml_solde(&page)?;
    let total_des_operations = total_des_operations_of_page(&page);
    dbg!(&total_des_operations);

    let texts: Vec<&Text> = match &page.items {
        None => vec![],
        Some(v) => v
            .iter()
            .map(|item| match item {
                Item::Text_(t) => Some(t),
                _ => None,
            })
            .filter(|t| t.is_some())
            .map(|t| t.unwrap())
            .collect(),
    };
    let date_column = find_column(&texts, "D", "ate")?;
    // dbg!(&date_column);

    let nature_column = find_column(&texts, "N", "ature des opérations")?;
    // dbg!(&nature_column);

    let valeur_column = find_column(&texts, "V", "aleur")?;
    // dbg!(&valeur_column);

    let debit_column = find_column(&texts, "D", "ébit")?;
    // dbg!(&debit_column);

    let credit_column = find_column(&texts, "C", "rédit")?;
    // dbg!(&credit_column);

    let date_rows: Vec<_> = texts
        .iter()
        .filter(|t| t.top > date_column.top)
        .filter(|t| (t.left as i32 - date_column.left as i32).abs() < 2)
        .filter(|t| match &total_des_operations {
            None => true,
            Some(total) => t.top < total.top,
        })
        .collect();
    // dbg!(&date_rows);

    let left_of_nature = nature_column.left;
    let left_of_valeur = valeur_column.left;
    let right_of_credit = credit_column.left + credit_column.width;
    let right_of_debit = debit_column.left + debit_column.width;

    let rows: Result<Vec<_>, _> = date_rows
        .iter()
        .map(|r| {
            let rr = r.clone();
            let rrr = rr.clone();
            let rrrr = rrr.clone();
            let r = row_of_date(
                rrrr,
                &texts,
                ec,
                left_of_nature,
                left_of_valeur,
                right_of_credit,
                right_of_debit,
                &total_des_operations,
            );
            if r.is_err() {
                dbg!(&r);
            }
            r
        })
        .collect();
    let rows = rows?;

    Ok(Table {
        releve: releve,
        total_des_operations_credit: match &total_des_operations {
            None => 0,
            Some(s) => s.credit,
        },
        total_des_operations_debit: match &total_des_operations {
            None => 0,
            Some(s) => s.debit,
        },
        rows: rows,
    })
}
