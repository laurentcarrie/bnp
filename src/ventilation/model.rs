use crate::parser::model::Operation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Assignment {
    pub name: String,
    pub patterns: Vec<String>,
    #[serde(default)]
    pub ignore: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VentilationSpec {
    pub name: String,
    pub assignments: Vec<Assignment>,
}

#[derive(Debug, Serialize)]
pub struct Ventilation {
    pub spec: VentilationSpec,
    pub ventilation: HashMap<String, i64>,
    pub not_assigned: i64,
    pub ventilated_operations: HashMap<String, Vec<Operation>>,
    pub not_assigned_operations: Vec<Operation>,
}
