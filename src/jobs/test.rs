use std::fs;

use chrono::NaiveDate;
use serde_xml_rs::from_str;

use crate::jobs::enclosing_dates::enclosing_dates_of_xml;
use crate::jobs::parse_page::parse_page;
use crate::jobs::scan::scan;
use crate::jobs::solde::{get_xml_solde_after, get_xml_solde_before};
use crate::jobs::xml_model::{Fontspec, Page, Pdf2xml, Text};
use crate::util::error::MyError;

// use crate::util::error::MyError;

#[test]
fn test_montant_before() -> Result<(), String> {
    let out_path = "RLV_CHQ_300040079300004047403_20240116.xml";
    let data = fs::read_to_string(out_path).unwrap();
    let p: Pdf2xml = from_str(&data).unwrap();
    let s = get_xml_solde_before(&p).unwrap();
    assert_eq!(s.montant, 149428);
    Ok(())
}

#[test]
fn test_montant_after() -> Result<(), String> {
    use crate::jobs::xml_model::Pdf2xml;
    use serde_xml_rs::from_str;
    use std::fs;
    let out_path = "RLV_CHQ_300040079300004047403_20240116.xml";
    let data = fs::read_to_string(out_path).unwrap();
    let p: Pdf2xml = from_str(&data).unwrap();
    let s = get_xml_solde_after(&p).unwrap();
    assert_eq!(s.montant, -117738);
    Ok(())
}

#[test]
fn test() -> Result<(), String> {
    use serde_xml_rs::from_str;
    let document = r#"
<text top="81" left="622" width="30" height="2" font="0">2024</text>"#;
    let t: Text = from_str(document).unwrap();
    assert_eq!(t.top, 81);
    assert_eq!(t.value, "2024");
    Ok(())
}

#[test]
fn test_1() -> Result<(), String> {
    use serde_xml_rs::from_str;
    let document = r###"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE pdf2xml SYSTEM "pdf2xml.dtd">
<pdf2xml producer="poppler" version="22.02.0">
<page number="1" position="absolute" top="0" left="0" height="1263" width="892">
</page>
</pdf2xml>
"###;
    let _t: Pdf2xml = from_str(document).unwrap();
    Ok(())
}

#[test]
fn test_2() -> Result<(), String> {
    use serde_xml_rs::from_str;
    let document = r###"<fontspec id="0" size="1" family="Times" color="#000000"/>
"###;
    let _t: Fontspec = from_str(document).unwrap();
    Ok(())
}

#[test]
fn test_3() -> Result<(), String> {
    use serde_xml_rs::from_str;
    let document = r###"<page number="1" position="absolute" top="0" left="0" height="1263" width="892">
    <fontspec id="0" size="1" family="Times" color="#000000"/>
</page>
"###;
    let _t: Page = from_str(document).unwrap();
    Ok(())
}

#[test]
fn test_4() -> Result<(), String> {
    use serde_xml_rs::from_str;
    let document = r###"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE pdf2xml SYSTEM "pdf2xml.dtd">
<pdf2xml producer="poppler" version="22.02.0">
<page number="1" position="absolute" top="0" left="0" height="1263" width="892">
<fontspec id="0" size="1" family="Times" color="#000000"/>
</page>
</pdf2xml>
"###;
    let _t: Pdf2xml = from_str(document).unwrap();
    Ok(())
}

#[test]
fn test_full() -> Result<(), String> {
    use serde_xml_rs::from_str;
    let document = r###"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE pdf2xml SYSTEM "pdf2xml.dtd">
<pdf2xml producer="poppler" version="22.02.0">
<page number="1" position="absolute" top="0" left="0" height="1263" width="892">
<fontspec id="0" size="1" family="Times" color="#000000"/>
<image top="93" left="0" width="251" height="-92" src="RLV_CHQ_300040079300004047403_20240116-1_1.png"/>
<image top="1255" left="846" width="40" height="-159" src="RLV_CHQ_300040079300004047403_20240116-1_2.png"/>
<text top="292" left="531" width="254" height="2" font="0">92250 LA GARENNE COLOMBES</text>
<text top="275" left="531" width="142" height="2" font="0">22 AVENUE FOCH</text>
</page>
</pdf2xml>
"###;
    let _t: Pdf2xml = from_str(document).unwrap();
    Ok(())
}

#[test]
fn test_page_0() -> Result<(), MyError> {
    let out_path = "RLV_CHQ_300040079300004047403_20240116.xml";
    let data = fs::read_to_string(out_path)?;
    let t: Pdf2xml = from_str(&data)?;
    eprintln!("before parse");
    let page = t.pages.get(0).expect("at least one page");
    let x = parse_page(page)?;
    // dbg!("{}", &x.rows);
    assert_eq!((&x.rows).len(), 5);
    assert_eq!(x.rows.get(0).unwrap().value, -7500);
    assert_eq!(x.rows.get(1).unwrap().value, -7850);
    Ok(())
}

#[test]
fn test_page_1() -> Result<(), MyError> {
    let out_path = "RLV_CHQ_300040079300004047403_20240116.xml";
    let data = fs::read_to_string(out_path).unwrap();
    let t: Pdf2xml = from_str(&data).unwrap();
    let x = parse_page(&t.pages.get(1).unwrap())?;
    assert_eq!((&x.rows).len(), 28);
    assert_eq!(x.rows.get(2).unwrap().value, 50000);
    Ok(())
}

#[test]
fn test_page_last() -> Result<(), MyError> {
    let out_path = "RLV_CHQ_300040079300004047403_20240116.xml";
    let data = fs::read_to_string(out_path).unwrap();
    let t: Pdf2xml = from_str(&data).unwrap();
    let page = t.pages.get(4).unwrap();
    // dbg!(&page);
    let x = parse_page(page)?;
    // dbg!(&x);
    assert_eq!((&x.rows).len(), 12);
    assert_eq!(x.rows.get(11).unwrap().value, -57277);
    Ok(())
}

#[test]
fn test_doc_0() -> Result<(), MyError> {
    let in_path = "RLV_CHQ_300040079300004047403_20240116.pdf";
    let table = scan(in_path.to_string())?;
    let _json = serde_json::to_string(&table).unwrap();
    // fs::write("RLV_CHQ_300040079300004047403_20240116.json", &json).unwrap();

    Ok(())
}

#[test]
fn test_doc_1() -> Result<(), MyError> {
    let in_path = "RLV_CHQ_300040079300004047403_20240116.pdf";
    let table = scan(in_path.to_string())?;
    dbg!(&table);
    assert_eq!(table.rows.len(), 104);
    assert_eq!(table.rows.get(103).unwrap().value, -57277);
    let (credit, debit) = table.rows.iter().fold((0, 0), |(credit, debit), row| {
        if row.value > 0 {
            (credit + row.value, debit)
        } else {
            (credit, debit + row.value)
        }
    });
    assert_eq!(credit, 436139);
    assert_eq!(debit, -703305);
    Ok(())
}

#[test]
fn test_enclosing_dates() -> Result<(), MyError> {
    let out_path = "RLV_CHQ_300040079300004047403_20240116.xml";
    let data = fs::read_to_string(out_path).unwrap();
    let t: Pdf2xml = from_str(&data).unwrap();
    let (s1, s2) = enclosing_dates_of_xml(&t)?;
    let nd1 = NaiveDate::from_ymd_opt(2023, 12, 13).unwrap();
    assert_eq!(s1, nd1);
    let nd2 = NaiveDate::from_ymd_opt(2024, 1, 13).unwrap();
    assert_eq!(s2, nd2);
    Ok(())
}
