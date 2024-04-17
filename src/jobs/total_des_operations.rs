use crate::jobs::model::TotalDesOperations;
use crate::jobs::solde::get_texts_of_page;
use crate::jobs::xml_model::{Page, Pdf2xml};
use crate::util::error::MyError;

pub fn total_des_operations_of_page(page: &Page) -> Option<TotalDesOperations> {
    let texts = get_texts_of_page(&page);
    let found: Vec<u32> = texts
        .iter()
        .filter_map(|row| match row.value.as_str() {
            "TOTAL DES OPERATIONS" => Some(row.top),
            _ => None,
        })
        .collect();
    if found.is_empty() {
        return None;
    }
    let top = found.get(0).unwrap();

    let mut all = texts
        .iter()
        .filter_map(|row| if row.top == *top { Some(row) } else { None })
        .collect::<Vec<_>>();

    all.sort_by(|a, b| a.left.cmp(&b.left));

    // dbg!(&all);
    let debit: u32 = all
        .get(1)?
        .value
        .replace(" ", "")
        .replace(",", "")
        .parse::<u32>()
        .ok()?;
    let credit: u32 = all
        .get(2)?
        .value
        .replace(" ", "")
        .replace(",", "")
        .parse::<u32>()
        .ok()?;

    Some(TotalDesOperations {
        debit: debit,
        credit: credit,
        top: *top,
        page_number: page.number,
    })
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
}
