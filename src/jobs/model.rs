use chrono::NaiveDate;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TotalDesOperations {
    pub debit: i32,
    pub credit: i32,
    pub top: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Poste {
    pub nom: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Row {
    pub date: NaiveDate,
    pub nature: String,
    pub value: i32,
    pub postes: Vec<Poste>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Table {
    pub rows: Vec<Row>,
}
