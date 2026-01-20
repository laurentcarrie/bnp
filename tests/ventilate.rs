use chrono::NaiveDate;
use mybnp::parser::model::{Operation, Releve, Solde, SoldeType};
use mybnp::ventilation::model::{Assignment, VentilationSpec};
use mybnp::ventilation::ventilate::{VentilateError, ventilate};

#[test]
fn test_ventilate() {
    let spec = VentilationSpec {
        name: "Test".to_string(),
        assignments: vec![
            Assignment {
                name: "Cirque".to_string(),
                patterns: vec!["CIRQUE".to_string()],
                ignore: false,
            },
            Assignment {
                name: "Restaurant".to_string(),
                patterns: vec!["RESTAURANT".to_string(), "REST\\.".to_string()],
                ignore: false,
            },
        ],
    };

    let releve = Releve {
        date_du_releve: NaiveDate::from_ymd_opt(2024, 1, 13).unwrap(),
        solde_ouverture: Solde {
            solde_type: SoldeType::Credit,
            montant: 100000,
        },
        solde_cloture: Solde {
            solde_type: SoldeType::Credit,
            montant: 90000,
        },
        total_des_operations_debit: 8000,
        total_des_operations_credit: 200000,
        check_debit: 8000,
        check_credit: 200000,
        operations: vec![
            Operation {
                date: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
                nature_des_operations: "CIRQUE DU SOLEIL".to_string(),
                valeur: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
                montant: 5000,
                montant_type: SoldeType::Debit,
            },
            Operation {
                date: NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
                nature_des_operations: "RESTAURANT CHEZ PAUL".to_string(),
                valeur: NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
                montant: 3000,
                montant_type: SoldeType::Debit,
            },
            Operation {
                date: NaiveDate::from_ymd_opt(2024, 1, 12).unwrap(),
                nature_des_operations: "SALAIRE".to_string(),
                valeur: NaiveDate::from_ymd_opt(2024, 1, 12).unwrap(),
                montant: 200000,
                montant_type: SoldeType::Credit,
            },
        ],
    };

    let result = ventilate(spec, &[releve]).unwrap();

    assert_eq!(result.ventilation.get("Cirque"), Some(&5000));
    assert_eq!(result.ventilation.get("Restaurant"), Some(&3000));
    assert_eq!(result.ventilation.get("Salaire"), None);
}

#[test]
fn test_ventilate_multiple_match_error() {
    let spec = VentilationSpec {
        name: "Test".to_string(),
        assignments: vec![
            Assignment {
                name: "Cirque".to_string(),
                patterns: vec!["CIRQUE".to_string()],
                ignore: false,
            },
            Assignment {
                name: "Soleil".to_string(),
                patterns: vec!["SOLEIL".to_string()],
                ignore: false,
            },
        ],
    };

    let releve = Releve {
        date_du_releve: NaiveDate::from_ymd_opt(2024, 1, 13).unwrap(),
        solde_ouverture: Solde {
            solde_type: SoldeType::Credit,
            montant: 100000,
        },
        solde_cloture: Solde {
            solde_type: SoldeType::Credit,
            montant: 95000,
        },
        total_des_operations_debit: 5000,
        total_des_operations_credit: 0,
        check_debit: 5000,
        check_credit: 0,
        operations: vec![Operation {
            date: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
            nature_des_operations: "CIRQUE DU SOLEIL".to_string(),
            valeur: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
            montant: 5000,
            montant_type: SoldeType::Debit,
        }],
    };

    let result = ventilate(spec, &[releve]);
    assert!(result.is_err());
    match result.unwrap_err() {
        VentilateError::MultipleMatch { operation, matches } => {
            assert_eq!(operation, "CIRQUE DU SOLEIL");
            assert!(matches.iter().any(|m| m.assignment == "Cirque"));
            assert!(matches.iter().any(|m| m.assignment == "Soleil"));
        }
        _ => panic!("Expected MultipleMatch error"),
    }
}

#[test]
fn test_ventilate_three_matches_error() {
    let spec = VentilationSpec {
        name: "Test".to_string(),
        assignments: vec![
            Assignment {
                name: "Cirque".to_string(),
                patterns: vec!["CIRQUE".to_string()],
                ignore: false,
            },
            Assignment {
                name: "Soleil".to_string(),
                patterns: vec!["SOLEIL".to_string()],
                ignore: false,
            },
            Assignment {
                name: "Du".to_string(),
                patterns: vec!["DU".to_string()],
                ignore: false,
            },
        ],
    };

    let releve = Releve {
        date_du_releve: NaiveDate::from_ymd_opt(2024, 1, 13).unwrap(),
        solde_ouverture: Solde {
            solde_type: SoldeType::Credit,
            montant: 100000,
        },
        solde_cloture: Solde {
            solde_type: SoldeType::Credit,
            montant: 95000,
        },
        total_des_operations_debit: 5000,
        total_des_operations_credit: 0,
        check_debit: 5000,
        check_credit: 0,
        operations: vec![Operation {
            date: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
            nature_des_operations: "CIRQUE DU SOLEIL".to_string(),
            valeur: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
            montant: 5000,
            montant_type: SoldeType::Debit,
        }],
    };

    let result = ventilate(spec, &[releve]);
    assert!(result.is_err());
    match result.unwrap_err() {
        VentilateError::MultipleMatch { matches, .. } => {
            assert_eq!(matches.len(), 3);
        }
        _ => panic!("Expected MultipleMatch error"),
    }
}

#[test]
fn test_ventilate_error_display() {
    let spec = VentilationSpec {
        name: "Test".to_string(),
        assignments: vec![
            Assignment {
                name: "A".to_string(),
                patterns: vec!["TEST".to_string()],
                ignore: false,
            },
            Assignment {
                name: "B".to_string(),
                patterns: vec!["TEST".to_string()],
                ignore: false,
            },
        ],
    };

    let releve = Releve {
        date_du_releve: NaiveDate::from_ymd_opt(2024, 1, 13).unwrap(),
        solde_ouverture: Solde {
            solde_type: SoldeType::Credit,
            montant: 100000,
        },
        solde_cloture: Solde {
            solde_type: SoldeType::Credit,
            montant: 95000,
        },
        total_des_operations_debit: 5000,
        total_des_operations_credit: 0,
        check_debit: 5000,
        check_credit: 0,
        operations: vec![Operation {
            date: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
            nature_des_operations: "TEST OPERATION".to_string(),
            valeur: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
            montant: 5000,
            montant_type: SoldeType::Debit,
        }],
    };

    let result = ventilate(spec, &[releve]);
    let err = result.unwrap_err();
    let msg = format!("{}", err);
    assert!(msg.contains("TEST OPERATION"));
    assert!(msg.contains("multiple assignments"));
}

#[test]
fn test_ventilate_error_on_second_operation() {
    let spec = VentilationSpec {
        name: "Test".to_string(),
        assignments: vec![
            Assignment {
                name: "Cirque".to_string(),
                patterns: vec!["CIRQUE".to_string()],
                ignore: false,
            },
            Assignment {
                name: "Soleil".to_string(),
                patterns: vec!["SOLEIL".to_string()],
                ignore: false,
            },
        ],
    };

    let releve = Releve {
        date_du_releve: NaiveDate::from_ymd_opt(2024, 1, 13).unwrap(),
        solde_ouverture: Solde {
            solde_type: SoldeType::Credit,
            montant: 100000,
        },
        solde_cloture: Solde {
            solde_type: SoldeType::Credit,
            montant: 90000,
        },
        total_des_operations_debit: 10000,
        total_des_operations_credit: 0,
        check_debit: 10000,
        check_credit: 0,
        operations: vec![
            Operation {
                date: NaiveDate::from_ymd_opt(2024, 1, 3).unwrap(),
                nature_des_operations: "RESTAURANT".to_string(),
                valeur: NaiveDate::from_ymd_opt(2024, 1, 3).unwrap(),
                montant: 5000,
                montant_type: SoldeType::Debit,
            },
            Operation {
                date: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
                nature_des_operations: "CIRQUE DU SOLEIL".to_string(),
                valeur: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
                montant: 5000,
                montant_type: SoldeType::Debit,
            },
        ],
    };

    let result = ventilate(spec, &[releve]);
    assert!(result.is_err());
    match result.unwrap_err() {
        VentilateError::MultipleMatch { operation, .. } => {
            assert_eq!(operation, "CIRQUE DU SOLEIL");
        }
        _ => panic!("Expected MultipleMatch error"),
    }
}

#[test]
fn test_ventilate_not_assigned() {
    let spec = VentilationSpec {
        name: "Test".to_string(),
        assignments: vec![Assignment {
            name: "Cirque".to_string(),
            patterns: vec!["CIRQUE".to_string()],
            ignore: false,
        }],
    };

    let releve = Releve {
        date_du_releve: NaiveDate::from_ymd_opt(2024, 1, 13).unwrap(),
        solde_ouverture: Solde {
            solde_type: SoldeType::Credit,
            montant: 100000,
        },
        solde_cloture: Solde {
            solde_type: SoldeType::Credit,
            montant: 80000,
        },
        total_des_operations_debit: 20000,
        total_des_operations_credit: 0,
        check_debit: 20000,
        check_credit: 0,
        operations: vec![
            Operation {
                date: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
                nature_des_operations: "CIRQUE DU SOLEIL".to_string(),
                valeur: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
                montant: 5000,
                montant_type: SoldeType::Debit,
            },
            Operation {
                date: NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(),
                nature_des_operations: "RESTAURANT".to_string(),
                valeur: NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(),
                montant: 3000,
                montant_type: SoldeType::Debit,
            },
            Operation {
                date: NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
                nature_des_operations: "SUPERMARCHE".to_string(),
                valeur: NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
                montant: 12000,
                montant_type: SoldeType::Debit,
            },
            Operation {
                date: NaiveDate::from_ymd_opt(2024, 1, 12).unwrap(),
                nature_des_operations: "SALAIRE".to_string(),
                valeur: NaiveDate::from_ymd_opt(2024, 1, 12).unwrap(),
                montant: 200000,
                montant_type: SoldeType::Credit,
            },
        ],
    };

    let result = ventilate(spec, &[releve]).unwrap();

    assert_eq!(result.ventilation.get("Cirque"), Some(&5000));
    assert_eq!(result.not_assigned, 15000); // 3000 + 12000 (RESTAURANT + SUPERMARCHE)
}

#[test]
fn test_ventilate_sum_mismatch() {
    let spec = VentilationSpec {
        name: "Test".to_string(),
        assignments: vec![Assignment {
            name: "Cirque".to_string(),
            patterns: vec!["CIRQUE".to_string()],
            ignore: false,
        }],
    };

    let releve = Releve {
        date_du_releve: NaiveDate::from_ymd_opt(2024, 1, 13).unwrap(),
        solde_ouverture: Solde {
            solde_type: SoldeType::Credit,
            montant: 100000,
        },
        solde_cloture: Solde {
            solde_type: SoldeType::Credit,
            montant: 95000,
        },
        total_des_operations_debit: 10000, // Incorrect: actual debits sum to 5000
        total_des_operations_credit: 0,
        check_debit: 10000,
        check_credit: 0,
        operations: vec![Operation {
            date: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
            nature_des_operations: "CIRQUE DU SOLEIL".to_string(),
            valeur: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
            montant: 5000,
            montant_type: SoldeType::Debit,
        }],
    };

    let result = ventilate(spec, &[releve]);
    assert!(result.is_err());
    match result.unwrap_err() {
        VentilateError::SumMismatch { expected, actual } => {
            assert_eq!(expected, 10000);
            assert_eq!(actual, 5000);
        }
        _ => panic!("Expected SumMismatch error"),
    }
}
