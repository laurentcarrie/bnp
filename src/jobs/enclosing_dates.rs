use std::collections::HashSet;

use chrono::NaiveDate;
use regex::Regex;

use crate::jobs::solde::get_texts_of_page;
use crate::jobs::xml_model::{Page, Pdf2xml, Text};
use crate::util::error::MyError;

pub fn intmonth_of_strmonth(month: &str) -> Result<u8, MyError> {
    match month {
        "janvier" => Ok(1),
        "février" => Ok(2),
        "mars" => Ok(3),
        "avril" => Ok(4),
        "mai" => Ok(5),
        "juin" => Ok(6),
        "juillet" => Ok(7),
        "août" => Ok(8),
        "septembre" => Ok(9),
        "octobre" => Ok(10),
        "novembre" => Ok(11),
        "décembre" => Ok(12),
        s => Err(MyError::Message(s.to_string())),
    }
}

pub fn enclosing_dates_of_page(page: &Page) -> Result<(NaiveDate, NaiveDate), MyError> {
    let texts = get_texts_of_page(&page);
    let texts: Vec<_> = texts.iter().filter(|t| t.top < 100).collect::<Vec<_>>();
    let du_hash: HashSet<i32> = texts
        .iter()
        .filter_map(|row| match row.value.as_str() {
            "du" => Some(row.top),
            _ => None,
        })
        .collect();

    let au_hash: HashSet<i32> = texts
        .iter()
        .filter_map(|row| match row.value.as_str() {
            "au" => Some(row.top),
            _ => None,
        })
        .collect();

    let intersection: HashSet<_> = du_hash
        .intersection(&au_hash)
        .map(|x| x.to_owned())
        .collect();

    match intersection.len() {
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
                .filter(|row| row.top == top)
                .collect::<Vec<&&Text>>();
            candidates.sort_by(|a, b| a.left.cmp(&b.left));
            let mut candidates = candidates
                .iter()
                .map(|t| t.value.clone())
                .collect::<Vec<_>>();
            let s = candidates.join(" ");
            let re = Regex::new(r"du *(\d+) *(\w+) *(\d+) *au *(\d+) *(\w+) *(\d+)").unwrap();
            let caps = re.captures(s.as_str());
            let caps = caps.unwrap();

            let day1 = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let month1 = intmonth_of_strmonth(caps.get(2).unwrap().as_str()).unwrap();
            let year1 = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
            let ss1 = format!("{:04}-{:02}-{:02}", year1, month1, day1);
            let nd1 = NaiveDate::parse_from_str(ss1.as_str(), "%Y-%m-%d")?;

            let day2 = caps.get(4).unwrap().as_str().parse::<u32>().unwrap();
            let month2 = intmonth_of_strmonth(caps.get(5).unwrap().as_str()).unwrap();
            let year2 = caps.get(6).unwrap().as_str().parse::<u32>().unwrap();
            let ss2 = format!("{:04}-{:02}-{:02}", year2, month2, day2);
            let nd2 = NaiveDate::parse_from_str(ss2.as_str(), "%Y-%m-%d")?;

            Ok((nd1, nd2))
        }
        _ => Err(MyError::Message("cannot find enclosing dates".to_string())),
    }
}

pub fn enclosing_dates_of_xml(xml: &Pdf2xml) -> Result<(NaiveDate, NaiveDate), MyError> {
    let r: Result<Vec<_>, MyError> = xml
        .pages
        .iter()
        .map(|p| enclosing_dates_of_page(p))
        .collect();
    let r = r?;
    let r = r.get(0).ok_or(MyError::Message("huh ?".to_string()));
    Ok(r?.clone())
}
