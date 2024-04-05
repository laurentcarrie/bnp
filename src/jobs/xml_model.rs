use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "pdf2xml")]
pub struct Text {
    pub top: i32,
    pub left: i32,
    pub width: i32,
    pub height: i32,
    pub font: String,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "fontspec")]
pub struct Fontspec {
    id: String,
    size: String,
    family: String,
    color: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "image")]
pub struct Image {
    top: String,
    left: String,
    width: String,
    height: String,
    src: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Item {
    #[serde(rename = "image")]
    Image_(Image),
    #[serde(rename = "text")]
    Text_(Text),
    #[serde(rename = "fontspec")]
    Fontspec_(Fontspec),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "pdf2xml")]
pub struct Pdf2xml {
    producer: String,
    version: String,
    #[serde(rename = "$value")]
    pub pages: Vec<Page>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "page")]
pub struct Page {
    pub number: i32,
    pub position: String,
    pub top: i32,
    pub left: i32,
    pub height: i32,
    pub width: i32,
    #[serde(rename = "$value")]
    pub items: Option<Vec<Item>>,
}
