use super::model::{Assignment, Ventilation, VentilationSpec};
use crate::parser::model::{Operation, Releve, SoldeType};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub enum VentilateError {
    MultipleMatch {
        operation: String,
        matches: Vec<String>,
    },
    SumMismatch {
        expected: f64,
        actual: f64,
    },
}

impl std::fmt::Display for VentilateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VentilateError::MultipleMatch { operation, matches } => {
                write!(
                    f,
                    "Operation '{operation}' matches multiple assignments: {matches:?}"
                )
            }
            VentilateError::SumMismatch { expected, actual } => {
                write!(
                    f,
                    "Sum mismatch: expected {expected} (total_des_operations_debit), got {actual} (ventilation + not_assigned)"
                )
            }
        }
    }
}

impl std::error::Error for VentilateError {}

fn find_matching_assignment<'a>(
    operation: &Operation,
    assignments: &'a [Assignment],
) -> Vec<&'a str> {
    let mut matches = Vec::new();
    for assignment in assignments {
        for pattern in &assignment.patterns {
            if let Ok(re) = Regex::new(pattern) {
                if re.is_match(&operation.nature_des_operations) {
                    matches.push(assignment.name.as_str());
                    break;
                }
            }
        }
    }
    matches
}

pub fn ventilate(spec: VentilationSpec, data: &[Releve]) -> Result<Ventilation, VentilateError> {
    let mut ventilation_map: HashMap<String, f64> = HashMap::new();
    let mut not_assigned = 0.0;

    for releve in data {
        for operation in &releve.operations {
            if let SoldeType::Debit = operation.montant_type {
                let matches = find_matching_assignment(operation, &spec.assignments);
                if matches.len() > 1 {
                    return Err(VentilateError::MultipleMatch {
                        operation: operation.nature_des_operations.clone(),
                        matches: matches.into_iter().map(String::from).collect(),
                    });
                }
                if let Some(name) = matches.first() {
                    *ventilation_map.entry(name.to_string()).or_insert(0.0) += operation.montant;
                } else {
                    not_assigned += operation.montant;
                }
            }
        }
    }

    let expected: f64 = data.iter().map(|r| r.total_des_operations_debit).sum();
    let actual: f64 = ventilation_map.values().sum::<f64>() + not_assigned;

    if (expected - actual).abs() > 0.01 {
        return Err(VentilateError::SumMismatch { expected, actual });
    }

    Ok(Ventilation {
        spec,
        ventilation: ventilation_map,
        not_assigned,
    })
}
