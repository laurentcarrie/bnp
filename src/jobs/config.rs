use crate::jobs::model::Config;
use crate::util::error::MyError;
use std::fs;

pub fn get_config() -> Result<Config, MyError> {
    let path = "config.json";
    if easy_paths::is_file(&path) {
        let data_json = fs::read_to_string(path)?;
        dbg!(&data_json);
        let config = serde_json::from_str::<Config>(&data_json)?;
        // Err(MyError::Message("not found".to_string()))
        Ok(config)
    } else {
        // Err(MyError::Message("not found".to_string()))
        Ok(Config {
            pdftohtml_path: None,
        })
    }
}
