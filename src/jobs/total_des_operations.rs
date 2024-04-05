use crate::jobs::model::TotalDesOperations;
use crate::jobs::solde::get_texts_of_page;
use crate::jobs::xml_model::{Page, Pdf2xml, Text};
use crate::util::error::MyError;
use regex::Regex;
use std::collections::HashSet;

pub fn total_des_operations_of_page(page: &Page) -> Option<TotalDesOperations> {
    let texts = get_texts_of_page(&page);
    let total_hash: HashSet<i32> = texts
        .iter()
        .filter_map(|row| match row.value.as_str() {
            "TOTAL" => Some(row.top),
            _ => None,
        })
        .collect();
    if total_hash.is_empty() {
        return None;
    }

    let des_hash: HashSet<i32> = texts
        .iter()
        .filter_map(|row| match row.value.as_str() {
            "DES" => Some(row.top),
            _ => None,
        })
        .collect();

    let operations_hash: HashSet<i32> = texts
        .iter()
        .filter_map(|row| match row.value.as_str() {
            "OPERATIONS" => Some(row.top),
            _ => None,
        })
        .collect();

    let intersection: HashSet<_> = total_hash
        .intersection(&des_hash)
        .map(|x| x.to_owned())
        .collect();
    let intersection: HashSet<_> = intersection.intersection(&operations_hash).collect();
    // let result :HashSet<i32>= total_hash.intersection(&des_hash).collect() ;
    // dbg!(&result) ;

    match intersection.len() {
        0 => None,
        1 => {
            let top = intersection
                .iter()
                .collect::<Vec<_>>()
                .get(0)
                .unwrap()
                .clone()
                .to_owned();
            let mut candidates = texts
                .iter()
                .filter(|row| row.top == *top)
                .collect::<Vec<&Text>>();
            candidates.sort_by(|a, b| a.left.cmp(&b.left));
            let mut candidates = candidates
                .iter()
                .map(|t| t.value.clone())
                .collect::<Vec<_>>();
            candidates.drain(0..3);
            let s = candidates.join(" ");
            let re = Regex::new(r"([\d ]+,\d+) *(.*)").unwrap();
            let caps = re.captures(s.as_str())?;
            let debit: i32 = caps
                .get(1)?
                .as_str()
                .replace(" ", "")
                .replace(",", "")
                .parse::<i32>()
                .ok()?;
            let credit: i32 = caps
                .get(2)?
                .as_str()
                .replace(" ", "")
                .replace(",", "")
                .parse::<i32>()
                .ok()?;
            Some(TotalDesOperations {
                debit: debit,
                credit: credit,
                top: *top,
            })
        }
        _ => None,
    }
}

pub fn total_des_operations_of_xml(xml: &Pdf2xml) -> Result<TotalDesOperations, MyError> {
    let r: Vec<TotalDesOperations> = xml
        .pages
        .iter()
        .filter_map(|p| total_des_operations_of_page(p))
        .collect();
    match r.len() {
        1 => {
            let t: TotalDesOperations = r.get(0).unwrap().clone();
            Ok(t)
        }
        n => Err(MyError::Message(format!(
            "found {} total des operations",
            n
        ))),
    }
    // Err(MyError::Message("blah blah".to_string()))
}
