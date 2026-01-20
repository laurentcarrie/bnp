use std::collections::HashMap;

#[derive(Debug)]
pub struct Assignment {
    pub name: String,
    pub patterns: Vec<String>,
}

#[derive(Debug)]
pub struct VentilationSpec {
    pub name: String,
    pub assignments: Vec<Assignment>,
}

#[derive(Debug)]
pub struct Ventilation {
    pub spec: VentilationSpec,
    pub ventilation: HashMap<String, f64>,
    pub not_assigned: f64,
}
