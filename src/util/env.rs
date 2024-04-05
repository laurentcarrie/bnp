use crate::util::error::MyError;
use std::env;

pub fn _get_env_var(name: &str) -> Result<String, MyError> {
    match env::var(name).ok() {
        None => Err(MyError::VarError(name.to_string())),
        Some(value) => Ok(value),
    }
}
