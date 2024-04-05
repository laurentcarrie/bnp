use crate::jobs::model::{Row, Table};
// use polars::prelude::*;
use crate::jobs::iomodel::load;

use crate::jobs::scan::scan;
use crate::util::error::MyError;

fn check_one_row(row1: &Row, others: &Table) -> Result<bool, MyError> {
    for row2 in &others.rows {
        if row2.nature == row1.nature && row2.value == row1.value && row2.date == row1.date {
            return Ok(true);
        }
    }
    Err(MyError::Message(format!(
        "petit probleme {} non trouve",
        row1.nature
    )))
}

pub fn check(pdfname: String) -> Result<bool, MyError> {
    let pdfname = pdfname.clone();
    let doc1 = scan(pdfname.clone())?;
    let jsonfilename = pdfname;
    let jsonfilename = jsonfilename.replace(".pdf", ".json");
    let doc2 = load(jsonfilename)?;

    for row in doc1.rows {
        match check_one_row(&row, &doc2) {
            Ok(true) => {
                ();
            }
            Ok(false) => {
                eprintln!("check row {}", row.nature);
            }
            Err(m) => {
                return Err(m);
            }
        }
    }

    Ok(false)
}
