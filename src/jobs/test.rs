#[cfg(test)]
use std::fs;

use chrono::NaiveDate;
use serde_xml_rs::from_str;

use crate::jobs::enclosing_dates::enclosing_dates_of_xml;
use crate::jobs::model::Value;
use crate::jobs::parse_page::parse_page;
use crate::jobs::scan::scan;
use crate::jobs::solde::{get_xml_solde_after, get_xml_solde_before};
use crate::jobs::xml_model::{Fontspec, Page, Pdf2xml, Text};
use crate::util::error::MyError;

#[test]
fn test_montant_before() -> Result<(), String> {
    let out_path = "RLV_CHQ_300040079300004047403_20240116.xml";
    let data = fs::read_to_string(out_path).unwrap();
    let p: Pdf2xml = from_str(&data).unwrap();
    let s = get_xml_solde_before(&p).unwrap();
    assert_eq!(s.montant, Value::Credit(149428));
    Ok(())
}

#[test]
fn test_montant_after() -> Result<(), String> {
    let out_path = "RLV_CHQ_300040079300004047403_20240116.xml";
    let data = fs::read_to_string(out_path).unwrap();
    let p: Pdf2xml = from_str(&data).unwrap();
    let s = get_xml_solde_after(&p).unwrap();
    assert_eq!(s.montant, Value::Debit(117738));
    Ok(())
}

#[test]
fn test() -> Result<(), String> {
    let document = r#"
<text top="81" left="622" width="30" height="2" font="0">2024</text>"#;
    let t: Text = from_str(document).unwrap();
    assert_eq!(t.top, 81);
    assert_eq!(t.value, "2024");
    Ok(())
}

#[test]
fn test_1() -> Result<(), String> {
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
    let document = r###"<fontspec id="0" size="1" family="Times" color="#000000"/>
"###;
    let _t: Fontspec = from_str(document).unwrap();
    Ok(())
}

#[test]
fn test_3() -> Result<(), String> {
    let document = r###"<page number="1" position="absolute" top="0" left="0" height="1263" width="892">
    <fontspec id="0" size="1" family="Times" color="#000000"/>
</page>
"###;
    let _t: Page = from_str(document).unwrap();
    Ok(())
}

#[test]
fn test_4() -> Result<(), String> {
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
    let nd = NaiveDate::parse_from_str("2024-01-01", "%Y-%m-%d")?;
    let x = parse_page(page, nd)?;
    // dbg!("{}", &x.rows);
    assert_eq!((&x.rows).len(), 5);
    assert_eq!(x.rows.get(0).unwrap().value, Value::Debit(7500));
    assert_eq!(x.rows.get(1).unwrap().value, Value::Debit(7850));
    Ok(())
}

#[test]
fn test_page_1() -> Result<(), MyError> {
    let out_path = "RLV_CHQ_300040079300004047403_20240116.xml";
    let data = fs::read_to_string(out_path).unwrap();
    let t: Pdf2xml = from_str(&data).unwrap();
    let nd = NaiveDate::parse_from_str("2024-01-01", "%Y-%m-%d")?;
    let x = parse_page(&t.pages.get(1).unwrap(), nd)?;
    assert_eq!((&x.rows).len(), 28);
    assert_eq!(x.rows.get(2).unwrap().value, Value::Credit(50000));
    Ok(())
}

#[test]
fn test_page_last() -> Result<(), MyError> {
    let out_path = "RLV_CHQ_300040079300004047403_20240116.xml";
    let data = fs::read_to_string(out_path).unwrap();
    let t: Pdf2xml = from_str(&data).unwrap();
    let page = t.pages.get(4).unwrap();
    // dbg!(&page);
    let nd = NaiveDate::parse_from_str("2024-01-01", "%Y-%m-%d")?;
    let x = parse_page(page, nd)?;
    // dbg!(&x);
    assert_eq!((&x.rows).len(), 12);
    assert_eq!(x.rows.get(11).unwrap().value, Value::Debit(57277));
    Ok(())
}

#[test]
fn test_doc_0() -> Result<(), MyError> {
    let in_path = "RLV_CHQ_300040079300004047403_20240116.pdf";
    let table = scan(in_path.to_string()).unwrap();
    let _json = serde_json::to_string(&table).unwrap();
    // fs::write("RLV_CHQ_300040079300004047403_20240116.json", &json).unwrap();

    Ok(())
}

#[test]
fn test_doc_1() -> Result<(), MyError> {
    let in_path = "RLV_CHQ_300040079300004047403_20240116.pdf";
    let table = scan(in_path.to_string())?;
    assert_eq!(table.rows.len(), 104);
    assert_eq!(table.rows.get(103).unwrap().value, Value::Debit(57277));
    let (credit, debit) = table
        .rows
        .iter()
        .fold((0, 0), |(credit, debit), row| match row.value {
            Value::Credit(v) => (credit + v, debit),
            Value::Debit(v) => (credit, debit + v),
        });
    let json = serde_json::to_string(&table)?;
    let _x = fs::write("hello.json", json)?;
    assert_eq!(credit, 436139);
    assert_eq!(debit, 703305);
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

#[test]
fn test_whole_nature() -> Result<(), MyError> {
    let in_path = "RLV_CHQ_300040079300004047403_20240116.pdf";
    let table = scan(in_path.to_string())?;
    dbg!(&table.rows);
    dbg!(&table.rows.len());
    dbg!(&table.rows.get(table.rows.len() - 1));
    {
        let nature = "PRLV SEPA HENNER GMC-HENNER-GMC ECH/151223 ID EMETTEUR/FR56ZZZ414162 MDT/H1162569942 REF/HEN001340433792 EN000066071820 LIB/HEN001340433792 EN000066071820";
        let index = 2;
        let row = table.rows.get(index).unwrap();
        assert_eq!(nature, row.nature.as_str());
    }
    {
        let nature = "PRLV SEPA 22/24 AV. FOCH - LA GARENNE C. ECH/120124 ID EMETTEUR/FR80ZZZ825BFD MDT/++W0403C000016568N000002562 REF/202401101649-2-72-5-04310040 LIB/PRL. SYNDIC CITYA GATFIC";
        let index = table.rows.len() - 1;
        let row = table.rows.get(index).unwrap();
        assert_eq!(nature, row.nature.as_str());
    }

    {
        // for i in 0..table.rows.len() {
        //     if table.rows.get(i).unwrap().nature.contains("NAVIGO") {
        //         println!("{}", i);
        //     }
        // }
        let nature="PRLV SEPA NAVIGO ANNUEL - COMUTITRES SAS ECH/030124 ID EMETTEUR/FR42ZZZ457385 MDT/FR42ZZZ45738550214077876357040044 REF/3/50214077876357/793678316 LIB/NAVIGO ANNUEL" ;
        let index = 92;
        let row = table.rows.get(index).unwrap();
        assert_eq!(nature, row.nature.as_str());
    }
    Ok(())
}
