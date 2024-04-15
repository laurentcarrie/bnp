use chrono::NaiveDate;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TotalDesOperations {
    pub debit: i32,
    pub credit: i32,
    pub top: i32,
}

// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
// pub struct Poste {
//     pub nom: String,
// }

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Row {
    pub date: NaiveDate,
    pub nature: String,
    pub value: i32,
    pub poste: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Table {
    pub releve: NaiveDate,
    pub total_des_operations_credit: i32,
    pub total_des_operations_debit: i32,
    pub rows: Vec<Row>,
}
