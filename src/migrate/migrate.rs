use crate::jobs::iomodel::{releve_of_path, save};
use crate::jobs::scan::scan;
use crate::util::error::MyError;
use polars::prelude::TemporalMethods;

fn migrate_one_file(path: &String) -> Result<(), MyError> {
    // dbg!(&path);
    let data_json = std::fs::read_to_string(path.as_str())?;
    let old_data = serde_json::from_str::<crate::migrate::old_iomodel::Table>(data_json.as_str())?;
    // dbg!(&old_data) ;

    let rows: Vec<crate::jobs::model::Row> = old_data
        .rows
        .iter()
        .map({
            |r| crate::jobs::model::Row {
                date: r.date.clone(),
                nature: r.nature.clone(),
                value: r.value,
                poste: r.postes.join(""),
            }
        })
        .collect();
    let pdf_path = path.replace(".json", ".pdf");
    let new_table = scan(pdf_path.to_string())?;
    let new_table = crate::jobs::model::Table {
        releve: new_table.releve,
        total_des_operations_credit: new_table.total_des_operations_credit,
        total_des_operations_debit: new_table.total_des_operations_debit,
        rows: rows.clone(),
    };
    //let path = path.replace(".json","-xxx.json") ;
    save(new_table, (&path).to_string())?;

    Ok(())
}

pub fn migrate(in_dir: String) -> Result<(), MyError> {
    let v = easy_paths::get_paths_in_dir(&in_dir)?;
    let v: Vec<String> = v
        .iter()
        .filter(|v| match easy_paths::get_extension(v) {
            Some(x) => x == "pdf".to_string(),
            _ => false,
        })
        .filter_map(|v| {
            let j = format!(
                "{}/{}.json",
                easy_paths::get_dir_name(&v).unwrap(),
                easy_paths::get_base_name(&v).unwrap().replace(".pdf", "")
            );
            // eprintln!("test {}",j) ;
            match easy_paths::is_file(&j) {
                true => Some(j),
                false => None,
            }
        })
        .collect();

    let result = v
        .iter()
        .map(|s| migrate_one_file(s))
        .collect::<Result<Vec<_>, _>>();
    let result = result?;
    Ok(())
}
