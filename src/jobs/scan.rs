use chrono::NaiveDate;
use std::fs;

use crate::jobs::config::get_config;
use rocket::form::validate::Contains;
use serde_xml_rs::from_str;
use uuid::Uuid;

// use polars::prelude::*;
use crate::jobs::iomodel::{releve_of_path, save};
use crate::jobs::model::{signed_value_of_value, Row, Table, Value};
use crate::jobs::parse_page::parse_page;
use crate::jobs::process::{run, ProcessInput};
use crate::jobs::solde::{get_xml_solde_after, get_xml_solde_before};
use crate::jobs::total_des_operations::total_des_operations_of_xml;
use crate::jobs::xml_model::Pdf2xml;
use crate::util::error::MyError;

pub fn pdftoxml(in_path: String) -> Result<String, MyError> {
    let uuid = Uuid::new_v4();
    let out_path = format!("pdf2xml-{}.xml", uuid);

    let config = get_config()?;
    dbg!(&config);
    let path = match config.pdftohtml_path {
        Some(p) => {
            format!("{}/pdftohtml", p)
        }
        None => "pdftohtml".to_string(),
    };
    let path: String = match std::env::consts::OS {
        "windows" => {
            format!("{}.exe", path)
        }
        _ => path,
    };
    dbg!(&path);
    if !easy_paths::is_file(&path) {
        return Err(MyError::Message(format!("no such file : {}", &path)));
    }
    let pi = ProcessInput {
        command: path,
        args: vec!["-xml".to_string(), in_path, out_path.clone()],
    };
    let _po = run(pi)?;
    let data = fs::read_to_string(out_path)?;
    Ok(data)
}

fn check_soldes(table: &Table, t: &Pdf2xml) -> Result<(), MyError> {
    let sb = match get_xml_solde_before(&t)?.montant {
        Value::Credit(u) => u as i32,
        Value::Debit(u) => -(u as i32),
    };
    let sa = match get_xml_solde_after(&t)?.montant {
        Value::Credit(u) => u as i32,
        Value::Debit(u) => -(u as i32),
    };
    let diff = sa - sb;
    let cumul = table
        .rows
        .iter()
        .fold(0, |acc, row| acc + signed_value_of_value(row.value.clone()));
    if diff != cumul {
        return Err(MyError::Message(format!(
            "in xml(pdf), solde after-solde before = {}, in json cumul = {}",
            diff, cumul
        )));
    }

    let total_des_operations = total_des_operations_of_xml(&t)?;
    let (cumul_credit, cumul_debit) =
        table
            .rows
            .iter()
            .fold((0, 0), |(acc_credit, acc_debit), row| match row.value {
                Value::Credit(v) => (acc_credit + v, acc_debit),
                Value::Debit(v) => (acc_credit, acc_debit + v),
            });

    if cumul_credit != total_des_operations.credit {
        return Err(MyError::Message(format!(
            "different cumul credit : {} ; {}",
            cumul_credit, total_des_operations.credit
        )));
    }
    if cumul_debit != total_des_operations.debit {
        return Err(MyError::Message(format!(
            "different cumul debit : {} ; {}",
            cumul_debit, total_des_operations.debit
        )));
    }
    Ok(())
}

fn work_xml(data: String, releve: NaiveDate) -> Result<Table, MyError> {
    // if in_path.contains(".xml") == false {
    //     return Err(MyError::VarError(in_path));
    // }
    // let data = fs::read_to_string(in_path.as_str())?;
    let t: Pdf2xml = from_str(&data)?;
    let total = total_des_operations_of_xml(&t)?;
    let tables: Result<Vec<Table>, MyError> = t
        .pages
        .iter()
        .filter(|p| p.number <= total.page_number)
        .map(|p| parse_page(p, releve))
        .collect();
    let tables = tables?;
    let mut rows: Vec<Row> = vec![];
    for mut table in tables {
        rows.append(&mut table.rows);
    }
    let table = Table {
        releve: releve,
        total_des_operations_credit: total.credit,
        total_des_operations_debit: total.debit,
        rows: rows,
    };
    // let _ = check_soldes(&table, &t)?;

    // let df: PolarsResult<DataFrame> = DataFrame::new(data);
    // let _ = fs::write(out_csv,data) ;

    Ok(table)
}

pub fn scan(in_path: String) -> Result<Table, MyError> {
    if in_path.contains(".pdf") == false {
        return Err(MyError::VarError(in_path));
    }
    let xml_string = pdftoxml(in_path.clone())?;
    let releve = releve_of_path(in_path)?;
    let table = work_xml(xml_string, releve)?;
    Ok(table)
}

fn work_one_file(in_path: &String) -> Result<bool, MyError> {
    eprintln!("--> {}", &in_path);
    let jsonpath = in_path.replace(".pdf", ".json");
    // eprintln!("jsonpath : {}", jsonpath);
    let table = scan(in_path.clone())?;
    // println!("parse done, now saving {}", jsonpath);
    let _x = save(table, jsonpath)?;
    Ok(true)
}
pub fn initjson(in_dir: String) -> Result<(), MyError> {
    let v = easy_paths::get_paths_in_dir(&in_dir)?;
    let v: Vec<&String> = v
        .iter()
        .filter(|v| match easy_paths::get_extension(v) {
            Some(x) => x == "pdf".to_string(),
            _ => false,
        })
        .filter(|v| {
            let j = format!(
                "{}/{}.json",
                easy_paths::get_dir_name(&v).unwrap(),
                easy_paths::get_base_name(&v).unwrap().replace(".pdf", "")
            );
            // eprintln!("test {}",j) ;
            !easy_paths::is_file(&j)
        })
        .collect();
    let results: Result<Vec<bool>, MyError> = v.iter().map(|f| work_one_file(f)).collect();
    let _results: Vec<_> = results?;
    Ok(())
}

// pub fn write_initial_json(in_path: String) -> Result<Table, MyError> {
//     let table = scan(in_path.clone())?;
//     let json = serde_json::to_string(&table)?;
//     let jsonpath = in_path.replace(".pdf", ".json");
//     fs::write(jsonpath, json)?;
//     Ok(table)
// }
