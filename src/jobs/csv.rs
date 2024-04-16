use crate::util::error::MyError;
use chrono::NaiveDate;
use easy_paths::get_paths_in_dir;
// use polars::prelude::{DataFrame, PolarsResult, Series};
use crate::jobs::iomodel::load;
use polars::prelude::*;
use rocket::serde::{Deserialize, Serialize};

use crate::jobs::model::{unsigned_value_of_value, Row as mRow, Table as mTable, Value};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Row {
    pub date: NaiveDate,
    pub nature: String,
    pub value: f32,
    pub poste: String,
    pub releve: NaiveDate,
    pub commentaire: String,
}

fn tocsv2(out_path: String, tables: &Vec<mTable>, credit: bool) -> Result<(), MyError> {
    let mut rows: Vec<Row> = vec![];
    for table in tables {
        for row in &table.rows {
            match (credit, &row.value) {
                (true, Value::Credit(v)) | (false, Value::Debit(v)) => {
                    let r = Row {
                        date: row.date.clone(),
                        nature: row.nature.clone(),
                        value: {
                            let f: f32 = *v as f32;
                            f / 100.0
                        },
                        poste: row.poste.clone(),
                        releve: table.releve.clone(),
                        commentaire: row.commentaire.clone(),
                    };

                    rows.push(r);
                }
                _ => {}
            }
        }
    }
    let dates: Vec<NaiveDate> = rows.iter().map(|r| r.date.clone()).collect();
    let s1 = Series::new("date", dates);

    let natures: Vec<String> = rows.iter().map(|r| r.nature.clone()).collect();
    let s2 = Series::new("nature", natures);

    let postes: Vec<String> = rows.iter().map(|r| r.poste.clone()).collect();
    let s3 = Series::new("poste", postes);

    let values: Vec<f32> = rows.iter().map(|r| r.value).collect();
    let s4 = Series::new(if credit { "credit" } else { "debit" }, values);

    let releves: Vec<NaiveDate> = rows.iter().map(|r| r.releve).collect();
    let s5 = Series::new("releve", releves);

    let commentaires: Vec<String> = rows.iter().map(|r| r.commentaire.clone()).collect();
    let s6 = Series::new("commentaire", commentaires);

    let mut df: DataFrame = DataFrame::new(vec![s1, s2, s3, s4, s5, s6])?;
    let mut file = std::fs::File::create(out_path)?;
    let _ = CsvWriter::new(&mut file).finish(&mut df)?;
    Ok(())
}

pub fn tocsv(in_dir: String) -> Result<(), MyError> {
    let v = get_paths_in_dir(&in_dir)?;
    let v: Vec<&String> = v
        .iter()
        .filter(|p| match easy_paths::get_extension(p) {
            Some(s) => s == "json".to_string(),
            None => false,
        })
        .collect();
    let tables: Result<Vec<crate::jobs::model::Table>, MyError> =
        v.iter().map(|p| load(p.to_string())).collect();
    let tables = tables?;

    let outpath_debit = format!("{}/debit.csv", in_dir);
    let _x = tocsv2(outpath_debit.clone(), &tables, false)?;

    let outpath_credit = format!("{}/credit.csv", in_dir);
    let _x = tocsv2(outpath_credit.clone(), &tables, true)?;

    Ok(())
}
