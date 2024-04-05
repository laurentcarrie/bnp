use crate::jobs::xml_model::Item::Text_;
use crate::jobs::xml_model::{Page, Pdf2xml, Text};
use crate::util::error::MyError;

pub struct Solde {
    pub date: String,
    pub montant: i32,
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
    let candidates: Vec<_> = texts.iter().filter(|t| t.value == "SOLDE").collect();
    let candidate = candidates.get(0);
    if candidate.is_none() {
        return Err(MyError::Message("no candidate".to_string()));
    }
    let candidate = candidate.expect("candidate");
    let mut others: Vec<_> = texts
        .iter()
        .filter(|text| (text.top - candidate.top).abs() < 3)
        .collect();
    others.sort_by(|a, b| a.left.cmp(&b.left));
    // dbg!(&others);
    if others.get(0).expect("0").value != "SOLDE" {
        return Err(MyError::Message("internal error 0".to_string()));
    };
    let credit_debit = &others.get(1).expect("1").value;
    let credit_bool = match credit_debit.as_str() {
        "CREDITEUR" => true,
        "DEBITEUR" => false,
        _ => return Err(MyError::Message("internal error 1".to_string())),
    };
    if others.get(2).expect("2").value != "AU" {
        return Err(MyError::Message("internal error".to_string()));
    };
    let date = others.get(3).expect("3");
    let mut montant = "".to_string();
    for i in 4..others.len() {
        montant = format!("{} {}", montant, others.get(i).expect("loop").value);
    }
    // dbg!(&montant);
    let montant = montant.replace(" ", "").replace(",", "");
    let montant = montant.parse::<i32>().expect("parse ok");
    let montant = if credit_bool { montant } else { -montant };

    Ok(Solde {
        date: date.value.clone(),
        montant: montant,
    })
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
