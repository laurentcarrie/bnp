use std::fmt;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

use polars::prelude::PolarsError;

#[derive(Debug)]
pub enum MyError {
    // EnvVarMissing(String),
    // We will defer to the parse error implementation for their error.
    // Supplying extra info requires adding more data to the type.
    VarError(String),
    Curl(curl::Error),
    // OtherCurlError(String),
    // PackageNotFoundError(String, String),
    FromUtf8Error,
    FmtError(std::fmt::Error),
    IoError(std::io::Error),
    // ProcessError(String),
    ParseIntErr(ParseIntError),
    TokioPostgres(tokio_postgres::Error),
    ChronoParseError(chrono::ParseError),
    PolarsError(PolarsError),
    SerdeXmlError(serde_xml_rs::Error),
    SerdeJsonError(serde_json::Error),
    Message(String),
}

impl From<curl::Error> for MyError {
    fn from(err: curl::Error) -> MyError {
        MyError::Curl(err)
    }
}

impl From<FromUtf8Error> for MyError {
    fn from(_err: FromUtf8Error) -> MyError {
        MyError::FromUtf8Error
    }
}

impl From<std::fmt::Error> for MyError {
    fn from(err: std::fmt::Error) -> MyError {
        MyError::FmtError(err)
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> MyError {
        MyError::IoError(err)
    }
}
impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> MyError {
        MyError::ParseIntErr(err)
    }
}

impl From<tokio_postgres::Error> for MyError {
    fn from(err: tokio_postgres::Error) -> MyError {
        MyError::TokioPostgres(err)
    }
}

impl From<chrono::ParseError> for MyError {
    fn from(err: chrono::ParseError) -> MyError {
        MyError::ChronoParseError(err)
    }
}
impl From<PolarsError> for MyError {
    fn from(err: PolarsError) -> MyError {
        MyError::PolarsError(err)
    }
}
impl From<serde_xml_rs::Error> for MyError {
    fn from(err: serde_xml_rs::Error) -> MyError {
        MyError::SerdeXmlError(err)
    }
}
impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> MyError {
        MyError::SerdeJsonError(err)
    }
}

impl From<String> for MyError {
    fn from(err: String) -> MyError {
        MyError::Message(err)
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::VarError(msg) => write!(f, "could not find env var {}", msg),
            MyError::Message(msg) => write!(f, "error : {}", msg),
            MyError::Curl(msg) => write!(f, "curl error {}", msg),
            MyError::FromUtf8Error => write!(f, "fromutf8error"),
            MyError::FmtError(err) => {
                write!(f, "{}", err)
            }
            MyError::IoError(err) => {
                write!(f, "{}", err)
            }
            // MyError::ProcessError(msg) => {
            //     write!(f, "process error {}", msg)
            // }
            MyError::ParseIntErr(msg) => {
                write!(f, "parse int error {}", msg)
            }
            MyError::TokioPostgres(msg) => {
                write!(f, "tokio postgres error {}", msg)
            }
            MyError::PolarsError(err) => write!(f, "polars error : {}", err),
            MyError::SerdeXmlError(err) => write!(f, "serde xml error : {}", err),
            MyError::SerdeJsonError(err) => write!(f, "serde json error : {}", err),
            MyError::ChronoParseError(err) => write!(f, "{}", err),
        }
    }
}
