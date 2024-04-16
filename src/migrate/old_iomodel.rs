use crate::jobs::model;
use crate::util::error::MyError;
use chrono::NaiveDate;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Row {
    pub date: NaiveDate,
    pub nature: String,
    pub value: i32,
    pub poste: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Table {
    pub rows: Vec<Row>,
}

// fn iorow_of_row(row: &model::Row) -> Row {
//     let postes = row.postes.iter().map(|p| p.nom.clone()).collect();
//
//     Row {
//         date: row.date.clone(),
//         nature: row.nature.clone(),
//         value: row.value,
//         postes: postes,
//     }
// }
//
// fn row_of_iorow(iorow: &Row) -> model::Row {
//     let postes = iorow
//         .postes
//         .iter()
//         .map(|r| model::Poste { nom: r.clone() })
//         .collect();
//
//     model::Row {
//         date: iorow.date.clone(),
//         nature: iorow.nature.clone(),
//         value: iorow.value,
//         poste: poste,
//     }
// }
