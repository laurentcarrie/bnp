use crate::jobs::model;
use crate::util::error::MyError;
use rocket::serde::{Deserialize, Serialize};
use std::fs;
use chrono::NaiveDate;
use crate::jobs::parse_page::naive_date_of_string;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Poste {
    pub nom: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Row {
    pub date: NaiveDate,
    pub nature: String,
    pub value: i32,
    pub postes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Table {
    pub rows: Vec<Row>,
}

fn iorow_of_row(row: &model::Row) -> Row {
    let postes = row.postes.iter().map(|p| p.nom.clone()).collect();

    Row {
        date: row.date.clone(),
        nature: row.nature.clone(),
        value: row.value,
        postes: postes,
    }
}

fn row_of_iorow(iorow: &Row) -> model::Row {
    let postes = iorow
        .postes
        .iter()
        .map(|r| model::Poste { nom: r.clone() })
        .collect();

    model::Row {
        date: iorow.date.clone(),
        nature: iorow.nature.clone(),
        value: iorow.value,
        postes: postes,
    }
}

pub fn save(table: model::Table, path: String) -> Result<(), MyError> {
    let rows: Vec<Row> = table
        .rows
        .into_iter()
        .map(|row| iorow_of_row(&row))
        .collect();
    let table = Table { rows: rows };
    let json = serde_json::to_string(&table)?;
    let _ = fs::write(path, json)?;
    Ok(())
}

pub fn load(path: String) -> Result<model::Table, MyError> {
    let data_json = std::fs::read_to_string(path.as_str())?;
    let iotable = serde_json::from_str::<Table>(data_json.as_str())?;

    let rows: Vec<model::Row> = iotable
        .rows
        .into_iter()
        .map(|row| row_of_iorow(&row))
        .collect();
    // # try_collect ???
    let table = model::Table { rows: rows };
    Ok(table)
}
