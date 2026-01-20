use super::model::{Assignment, Ventilation, VentilationSpec};
use crate::parser::model::{Operation, Releve, SoldeType};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct MatchInfo {
    pub assignment: String,
    pub pattern: String,
}

#[derive(Debug)]
pub enum VentilateError {
    MultipleMatch {
        operation: String,
        matches: Vec<MatchInfo>,
    },
    SumMismatch {
        expected: i64,
        actual: i64,
    },
}

impl std::fmt::Display for VentilateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VentilateError::MultipleMatch { operation, matches } => {
                writeln!(f, "Operation '{operation}' matches multiple assignments:")?;
                for m in matches {
                    writeln!(f, "  - {} (pattern: {})", m.assignment, m.pattern)?;
                }
                Ok(())
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

fn find_matching_assignment(operation: &Operation, assignments: &[Assignment]) -> Vec<MatchInfo> {
    let mut matches = Vec::new();
    for assignment in assignments {
        for pattern in &assignment.patterns {
            if let Ok(re) = Regex::new(pattern) {
                if re.is_match(&operation.nature_des_operations) {
                    matches.push(MatchInfo {
                        assignment: assignment.name.clone(),
                        pattern: pattern.clone(),
                    });
                    break;
                }
            }
        }
    }
    matches
}

pub fn ventilate(spec: VentilationSpec, data: &[Releve]) -> Result<Ventilation, VentilateError> {
    let mut ventilation_map: HashMap<String, i64> = HashMap::new();
    let mut ventilated_operations: HashMap<String, Vec<Operation>> = HashMap::new();
    let mut not_assigned: i64 = 0;
    let mut not_assigned_operations: Vec<Operation> = Vec::new();

    for releve in data {
        for operation in &releve.operations {
            if let SoldeType::Debit = operation.montant_type {
                let matches = find_matching_assignment(operation, &spec.assignments);
                if matches.len() > 1 {
                    return Err(VentilateError::MultipleMatch {
                        operation: operation.nature_des_operations.clone(),
                        matches,
                    });
                }
                if let Some(match_info) = matches.first() {
                    *ventilation_map
                        .entry(match_info.assignment.clone())
                        .or_insert(0) += operation.montant;
                    ventilated_operations
                        .entry(match_info.assignment.clone())
                        .or_default()
                        .push(operation.clone());
                } else {
                    not_assigned += operation.montant;
                    not_assigned_operations.push(operation.clone());
                }
            }
        }
    }

    let expected: i64 = data.iter().map(|r| r.total_des_operations_debit).sum();
    let actual: i64 = ventilation_map.values().sum::<i64>() + not_assigned;

    if expected != actual {
        return Err(VentilateError::SumMismatch { expected, actual });
    }

    Ok(Ventilation {
        spec,
        ventilation: ventilation_map,
        not_assigned,
        ventilated_operations,
        not_assigned_operations,
    })
}
