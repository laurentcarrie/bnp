use chrono::NaiveDate;
use crate::util::error::MyError;
use easy_paths::get_paths_in_dir;
// use polars::prelude::{DataFrame, PolarsResult, Series};
use crate::jobs::iomodel::load;
use crate::jobs::model::{Row, Table};
use polars::prelude::*;

pub fn tocsv(in_dir: String) -> Result<(), MyError> {
    let v = get_paths_in_dir(&in_dir)?;
    let v: Vec<&String> = v
        .iter()
        .filter(|p| match easy_paths::get_extension(p) {
            Some(s) => s == "json".to_string(),
            None => false,
        })
        .collect();
    let tables: Result<Vec<Table>, MyError> = v.iter().map(|p| load(p.to_string())).collect();
    let tables = tables?;
    dbg!(&tables);
    let mut rows: Vec<Row> = vec![];
    for mut table in tables {
        for row in table.rows {
            let r = row.clone();
            rows.push(r);
        }
    }
    let dates: Vec<NaiveDate> = rows.iter().map(|r| r.date.clone()).collect();
    let s1 = Series::new("date", dates);

    let natures: Vec<String> = rows.iter().map(|r| r.nature.clone()).collect();
    let s2 = Series::new("nature", natures);

    let values: Vec<i32> = rows.iter().map(|r| r.value).collect();
    let s3 = Series::new("value", values);

    let postes: Vec<String> = rows
        .iter()
        .map(|r| {
            r.postes
                .iter()
                .map(|p| p.nom.clone())
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect();
    let s4 = Series::new("postes", postes);

    let mut df: DataFrame = DataFrame::new(vec![s1, s2, s3, s4])?;

    let path = format!("{}/data.csv", in_dir);
    dbg!(&path);
    let mut file = std::fs::File::create(path).unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();
    Ok(())
}
