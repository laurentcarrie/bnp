use crate::jobs::model;
use crate::util::error::MyError;
use chrono::NaiveDate;
use regex::Regex;
use rocket::serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Value {
    Debit(u32),
    Credit(u32),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Row {
    pub date: NaiveDate,
    pub nature: String,
    pub value: Value,
    pub poste: String,
    pub commentaire: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Table {
    pub releve: NaiveDate,
    pub rows: Vec<Row>,
    pub total_des_operations_debit: u32,
    pub total_des_operations_credit: u32,
}

fn iorow_of_row(row: &model::Row) -> Row {
    Row {
        date: row.date.clone(),
        nature: row.nature.clone(),
        value: match row.value {
            model::Value::Credit(c) => Value::Credit(c),
            model::Value::Debit(c) => Value::Debit(c),
        },
        poste: row.poste.clone(),
        commentaire: row.commentaire.clone(),
    }
}

fn row_of_iorow(iorow: &Row) -> model::Row {
    model::Row {
        date: iorow.date.clone(),
        nature: iorow.nature.clone(),
        value: match iorow.value {
            Value::Credit(c) => model::Value::Credit(c),
            Value::Debit(c) => model::Value::Debit(c),
        },
        poste: iorow.poste.clone(),
        commentaire: iorow.commentaire.clone(),
    }
}

pub fn save(table: model::Table, path: String) -> Result<(), MyError> {
    let rows: Vec<Row> = table
        .rows
        .into_iter()
        .map(|row| iorow_of_row(&row))
        .collect();
    let table = Table {
        rows: rows,
        total_des_operations_debit: table.total_des_operations_debit,
        releve: table.releve,
        total_des_operations_credit: table.total_des_operations_credit,
    };
    let json = serde_json::to_string(&table)?;
    let _ = fs::write(path, json)?;
    Ok(())
}

pub fn releve_of_path(path: String) -> Result<NaiveDate, MyError> {
    let re = Regex::new(r"RLV_CHQ_(.*?)_(\d\d\d\d\d\d\d\d).pdf").unwrap();
    let caps = re
        .captures(&path)
        .ok_or(MyError::Message("xxx".to_string()))?;
    let _compte = caps.get(1).unwrap();
    let string_naivedateyear = caps.get(2).unwrap();
    let nd = NaiveDate::parse_from_str(string_naivedateyear.as_str(), "%Y%m%d")?;
    Ok(nd)
}

pub fn load(path: String) -> Result<model::Table, MyError> {
    let data_json = std::fs::read_to_string(path.as_str())?;
    let data = serde_json::from_str::<Table>(data_json.as_str())?;

    let rows: Vec<model::Row> = data
        .rows
        .into_iter()
        .map(|row| row_of_iorow(&row))
        .collect();

    let table = model::Table {
        releve: data.releve,
        total_des_operations_credit: data.total_des_operations_credit,
        total_des_operations_debit: data.total_des_operations_debit,
        rows: rows,
    };
    Ok(table)
}
