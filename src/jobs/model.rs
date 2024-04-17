use chrono::NaiveDate;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Config {
    pub pdftohtml_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TotalDesOperations {
    pub debit: u32,
    pub credit: u32,
    pub top: u32,
    pub page_number: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Value {
    Debit(u32),
    Credit(u32),
}

pub fn signed_value_of_value(v: Value) -> i32 {
    match v {
        Value::Debit(x) => -(x as i32),
        Value::Credit(x) => x as i32,
    }
}

pub fn _unsigned_value_of_value(v: Value) -> u32 {
    match v {
        Value::Debit(x) => x,
        Value::Credit(x) => x,
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Row {
    pub date: NaiveDate,
    pub nature: String,
    pub value: Value,
    pub poste: String,
    pub commentaire: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Table {
    pub releve: NaiveDate,
    pub total_des_operations_credit: u32,
    pub total_des_operations_debit: u32,
    pub rows: Vec<Row>,
}
