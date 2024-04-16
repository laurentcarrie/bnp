use chrono::{Datelike, NaiveDate};
use regex::Regex;

use crate::jobs::enclosing_dates::enclosing_dates_of_page;
use crate::jobs::model::{Row, Table, Value};
use crate::jobs::total_des_operations::total_des_operations_of_page;
use crate::jobs::xml_model::{Item, Page, Text};
use crate::util::error::MyError;

fn guess_poste(nature: String) -> String {
    if nature.contains("/MOTIF SALAIRE") {
        return "salaire".to_string();
    }
    if nature.starts_with("VIR SEPA RECU /DE CPAM") {
        return "secu".to_string();
    };
    if nature.starts_with("PRLV SEPA DGFIP IMPOT") {
        return "taxes".to_string();
    };

    if nature.contains("DU 041223 ATRIUM GARENNATRIUM/") {
        return "alimentation".to_string();
    };

    if nature.starts_with("PRLV SEPA HENNER GMC-HENNER-GMC") {
        return "mutuelle".to_string();
    };

    if nature.contains("EMIS /MOTIF") && nature.contains("IMMO") {
        return "emprunt_immo".to_string();
    };

    if nature.starts_with("PRLV SEPA NAVIGO ANNUEL") {
        return "navigo".to_string();
    };

    if nature.starts_with("PRLV SEPA CARDIF ASSURANCE VIE") {
        return "assurance_vie".to_string();
    };

    if nature.starts_with("PRLV SEPA MAIF") {
        return "maif".to_string();
    };

    if nature.starts_with("VIREMENT SEPA EMIS") && nature.contains("TAMARA") {
        return "virement_tam".to_string();
    };

    if nature.starts_with("RETRAIT DAB") {
        return "retrait_dab".to_string();
    };

    if nature.contains("GATFIC") {
        return "gatfic".to_string();
    };

    if nature.contains("LEROY MERLIN") {
        return "maison".to_string();
    };

    if nature.contains("LOYER PARKING") {
        return "box".to_string();
    };

    if nature.contains("EDF") {
        return "edf".to_string();
    };

    if nature.contains("ORANGE") {
        return "orange".to_string();
    };

    if nature.contains("dedibox") {
        return "orange".to_string();
    };

    if nature.starts_with("PRLV SEPA CONSERVATOIRE MUSIQUE") {
        return "conservatoire".to_string();
    };

    "?".to_string()
}
// pub fn last_page_index(xml: &Pdf2xml) -> Result<usize, MyError> {
//     for i in (0..xml.pages.len() - 1).rev() {
//         if is_last_page(xml.pages.get(i).unwrap()) {
//             return Ok(i);
//         }
//     }
//     Err(MyError::Message("could not find last page".to_string()))
// }

pub fn naive_date_of_string(
    s: &str,
    (nd1, nd2): (NaiveDate, NaiveDate),
) -> Result<NaiveDate, MyError> {
    let re = Regex::new(r"(\d+).(\d+)").unwrap();
    let caps = re.captures(s).ok_or(MyError::Message("xxx".to_string()))?;
    let day = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let month = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let year = if month == nd1.month() {
        nd1.year()
    } else if month == nd2.month() {
        nd2.year()
    } else {
        return Err(MyError::Message("internal".to_string()));
    };
    let ss = format!("{:04}-{:02}-{:02}", year, month, day);
    let nd = NaiveDate::parse_from_str(ss.as_str(), "%Y-%m-%d")?;
    Ok(nd)
}

pub fn split_to_cells(
    texts: Vec<&Text>,
    rows: Vec<i32>,
    cols: Vec<i32>,
) -> Result<Vec<Vec<String>>, String> {
    let mut result: Vec<Vec<String>> = vec![];
    for irow in 0..(rows.len() - 1) {
        let mut row: Vec<String> = vec![];
        let xmin = rows.get(irow).expect("internal").clone();
        let xmax = rows.get(irow + 1).expect("internal").clone();
        for icol in 0..(cols.len() - 1) {
            let ymin = cols.get(icol).expect("internal").clone();
            let ymax = cols.get(icol + 1).expect("internal").clone();
            // println!(
            //     "===================> {} => {} ; {} => {}",
            //     xmin, xmax, ymin, ymax
            // );
            let values: Vec<String> = texts
                .iter()
                .filter(|t| {
                    (t.left >= ymin) && (t.left < ymax) && (t.top >= xmin) && (t.top < xmax)
                })
                .map(|t| t.value.clone())
                .collect();
            let value = values.join(" ");
            // println!("    [{},{}] '{}'", irow, icol, value);
            row.push(value);
        }
        result.push(row);
    }

    // let s1 = Series::new("Date", &["Apple", "Apple", "Pear"]);
    // let s2 = Series::new("Nature", &["Red", "Yellow", "Green"]);
    //
    // let df: PolarsResult<DataFrame> = DataFrame::new(vec![s1, s2]);
    // println!("{:?}", df);
    // for row in rows {
    //
    // }
    return Ok(result);
}

pub fn parse_page(page: &Page, releve: NaiveDate) -> Result<Table, MyError> {
    let ec = enclosing_dates_of_page(page)?;
    let texts: Vec<&Text> = match &page.items {
        None => vec![],
        Some(v) => v
            .iter()
            .map(|item| match item {
                Item::Text_(t) => Some(t),
                _ => None,
            })
            .filter(|t| t.is_some())
            .map(|t| t.unwrap())
            .collect(),
    };
    let date_header: Vec<&&Text> = texts.iter().filter(|t| t.value == "Date").collect();
    dbg!(&date_header);
    // println!("found {} date", date_header.len());
    if date_header.len() != 1 {
        return Ok(Table {
            rows: vec![],
            releve: releve,
            total_des_operations_credit: 0,
            total_des_operations_debit: 0,
        });
        // return Err(format!("more or zero field Date : {}, page #{}",date_header.len(),page.number).to_string());
    }
    let date_header = date_header.get(0).ok_or("no 0 ???".to_string())?;
    let date_left: i32 = date_header.left - 1;
    let date_rows: Vec<&Text> = texts
        .iter()
        .filter(|t| t.left == date_left)
        .map(|t| t.clone())
        .collect();
    // println!("found {} date rows", date_rows.len());

    // let last_line: Vec<&&Text> = texts.iter().filter(|t| t.value == "TOTAL").collect();
    // if last_line.len() != 1 {
    //     return Err("more or zero field last line".to_string());
    // }

    let last_line_top = match total_des_operations_of_page(&page) {
        Some(t) => t.top,
        None => 1200,
    };

    let last_line: Text = Text {
        top: last_line_top,
        left: 0,
        width: 0,
        value: "".to_string(),
        height: 0,
        font: "".to_string(),
    };
    let last_line = &last_line;
    // dbg!("last line", &last_line);
    let mut date_rows: Vec<&&Text> = date_rows
        .iter()
        .filter(|row| row.top < last_line.top)
        .map(|row| row)
        .collect();
    date_rows.push(&last_line);

    let nature_header: Vec<&&Text> = texts
        .iter()
        .filter(|t| t.value == "Nature" && (t.top - date_header.top).abs() < 3)
        .collect();
    // println!("found {} nature", nature_header.len());
    if nature_header.len() != 1 {
        return Err(MyError::Message("more or zero field Nature".to_string()));
    }
    let nature_header = nature_header.get(0).ok_or("no 0 ???".to_string())?;

    let valeur_header: Vec<&&Text> = texts
        .iter()
        .filter(|t| t.value == "Valeur" && t.top == date_header.top)
        .collect();
    // println!("found {} value ", valeur_header.len());
    if valeur_header.len() != 1 {
        return Err(MyError::Message("more or zero field Valeur".to_string()));
    }
    let valeur_header = valeur_header.get(0).ok_or("no 0 ???".to_string())?;

    let valeur_debit: Vec<&&Text> = texts
        .iter()
        .filter(|t| t.value == "Débit" && (t.top - date_header.top).abs() < 3)
        .collect();
    // println!("found {} value ", valeur_debit.len());
    if valeur_debit.len() != 1 {
        return Err(MyError::Message("more or zero field Debit".to_string()));
    }
    let valeur_debit = valeur_debit.get(0).ok_or("no 0 ???".to_string())?;

    let valeur_credit: Vec<&&Text> = texts
        .iter()
        .filter(|t| t.value == "Crédit" && (t.top - date_header.top).abs() < 3)
        .collect();
    // println!("found {} value ", valeur_credit.len());
    if valeur_credit.len() != 1 {
        return Err(MyError::Message("more or zero field Credit".to_string()));
    }
    let valeur_credit = valeur_credit.get(0).ok_or("no 0 ???".to_string())?;

    let rows: Vec<i32> = date_rows.iter().map(|i| i.top - 1).collect();
    // dbg!("rows", &rows);
    let cols = vec![
        0,
        nature_header.left,
        valeur_header.left,
        valeur_header.left + valeur_header.width,
        valeur_debit.left + valeur_debit.width,
        valeur_credit.left + valeur_credit.width,
    ];

    let stringtable = split_to_cells(texts, rows, cols)?;
    let rows: Result<Vec<Row>, MyError> = stringtable
        .iter()
        .map(|row| {
            let date = row.get(0).expect("date").to_string();
            let nature = row.get(1).expect("nature").to_string();
            let debit = row
                .get(3)
                .expect("debit")
                .to_string()
                .replace(" ", "")
                .replace(",", "")
                .parse::<i32>();
            let credit = row
                .get(4)
                .expect("credit")
                .to_string()
                .replace(" ", "")
                .replace(",", "")
                .parse::<i32>();
            let value = match (credit, debit) {
                (Ok(v), Err(_)) => Value::Credit(v),
                (Err(_), Ok(v)) => Value::Debit(v),
                (Ok(_), Ok(_)) => {
                    return Err(MyError::Message("both debit and credit".to_string()));
                }
                (Err(_), Err(_)) => {
                    return Err(MyError::Message("neither debit nor credit".to_string()));
                }
            };
            let date = naive_date_of_string(date.as_str(), ec)?;
            Ok(Row {
                date: date,
                nature: nature.clone(),
                value: value,
                poste: guess_poste(nature),
                commentaire: "".to_string(),
            })
        })
        .collect();
    let rows = rows?;
    let (total_credit, total_debit) = match total_des_operations_of_page(&page) {
        Some(t) => (t.credit, t.debit),
        None => (0, 0),
    };
    let table = Table {
        releve: releve,
        total_des_operations_credit: total_credit,
        total_des_operations_debit: total_debit,
        rows: rows,
    };
    Ok(table)
}
