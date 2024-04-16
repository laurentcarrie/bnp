use chrono::NaiveDate;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TotalDesOperations {
    pub debit: i32,
    pub credit: i32,
    pub top: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Value {
    Debit(i32),
    Credit(i32),
}

pub fn signed_value_of_value(v: Value) -> i32 {
    match v {
        Value::Debit(x) => -x,
        Value::Credit(x) => x,
    }
}

pub fn unsigned_value_of_value(v: Value) -> i32 {
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
    pub total_des_operations_credit: i32,
    pub total_des_operations_debit: i32,
    pub rows: Vec<Row>,
}
