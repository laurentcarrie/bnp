use crate::jobs::model::Value;
use crate::jobs::xml_model::Item::Text_;
use crate::jobs::xml_model::{Page, Pdf2xml, Text};
use crate::util::error::MyError;
use chrono::NaiveDate;
use regex::Regex;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Solde {
    pub date: NaiveDate,
    pub montant: Value,
}

pub fn get_texts_of_page(page: &Page) -> Vec<Text> {
    let result: Vec<Text> = match &page.items {
        Some(items) => items
            .iter()
            .filter_map(|i| match i {
                Text_(t) => Some(t.clone()),
                _ => None,
            })
            .collect(),
        None => {
            vec![]
        }
    };
    result.clone()
}

pub fn get_xml_solde(page: &Page) -> Result<Solde, MyError> {
    let texts = get_texts_of_page(page);
    let candidates: Vec<_> = texts
        .iter()
        .filter(|t| t.value.starts_with("SOLDE"))
        .collect();
    let candidate = candidates.get(0);
    let candidate = candidate.ok_or(MyError::Message(
        "cannot find candidate for xml solde".to_string(),
    ))?;
    let re_credit = Regex::new(r"SOLDE CREDITEUR AU (\d\d\.\d\d\.\d\d\d\d)").unwrap();
    let re_debit = Regex::new(r"SOLDE DEBITEUR AU (\d\d\.\d\d\.\d\d\d\d)").unwrap();

    let cap_credit = re_credit.captures(&candidate.value.as_str());
    let cap_debit = re_debit.captures(&candidate.value.as_str());

    let other = texts
        .iter()
        .filter(|row| (row.top as i32 - candidate.top as i32).abs() < 3)
        .filter(|row| row != candidate)
        .next();
    let other = other.ok_or(MyError::Message(
        "cannot find candidate for xml solde".to_string(),
    ))?;
    let value = other.value.replace(" ", "").replace(",", "");
    let value = value.parse::<u32>()?;

    match (cap_credit, cap_debit) {
        (Some(s), None) => {
            let s = s.get(1).ok_or(MyError::Message("huh ? ".to_string()))?;
            let nd = NaiveDate::parse_from_str(s.as_str(), "%d.%m.%Y")?;
            Ok(Solde {
                date: nd,
                montant: Value::Credit(value),
            })
        }
        (None, Some(s)) => {
            let s = s.get(1).ok_or(MyError::Message("huh ?".to_string()))?;
            let nd = NaiveDate::parse_from_str(s.as_str(), "%d.%m.%Y")?;
            Ok(Solde {
                date: nd,
                montant: Value::Debit(value),
            })
        }
        _ => Err(MyError::Message("huh ?".to_string())),
    }
}

pub fn get_xml_solde_before(p: &Pdf2xml) -> Result<Solde, MyError> {
    let page = p.pages.get(0).expect("has one page");
    get_xml_solde(page)
}

pub fn get_xml_solde_after(p: &Pdf2xml) -> Result<Solde, MyError> {
    for i in (1..(p.pages.len())).rev() {
        let page = p.pages.get(i).expect("check number of pages");
        match get_xml_solde(page) {
            Ok(s) => {
                return Ok(s);
            }
            Err(_) => {}
        }
    }
    Err(MyError::Message("could not get solde after".to_string()))
}
