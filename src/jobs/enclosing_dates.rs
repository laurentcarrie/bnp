use chrono::NaiveDate;
use regex::Regex;

use crate::jobs::solde::get_texts_of_page;
use crate::jobs::xml_model::{Page, Pdf2xml};
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
    let texts: Vec<_> = texts
        .iter()
        .filter(|t| t.value.starts_with("du"))
        .collect::<Vec<_>>();
    let re = Regex::new(r"du *(\d+) *(\w+) *(\d+) *au *(\d+) *(\w+) *(\d+)").unwrap();
    let result: Vec<_> = texts
        .iter()
        .filter_map(|row| re.captures(row.value.as_str()))
        .collect();
    let result: Result<Vec<_>, MyError> = result
        .iter()
        .map(|caps| {
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

            Ok::<(NaiveDate, NaiveDate), MyError>((nd1, nd2))
        })
        .collect();
    let result = result?;
    match result.is_empty() {
        true => Err(MyError::Message(
            "could not find enclosing dates du .. au ... ".to_string(),
        )),
        false => Ok(*result.get(0).unwrap()),
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
