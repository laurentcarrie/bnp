use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SoldeType {
    Credit,
    Debit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub date: NaiveDate,
    pub nature_des_operations: String,
    pub valeur: NaiveDate,
    pub montant: i64,
    pub montant_type: SoldeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solde {
    pub solde_type: SoldeType,
    pub montant: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Releve {
    pub date_du_releve: NaiveDate,
    pub solde_ouverture: Solde,
    pub solde_cloture: Solde,
    pub total_des_operations_debit: i64,
    pub total_des_operations_credit: i64,
    pub check_debit: i64,
    pub check_credit: i64,
    pub operations: Vec<Operation>,
}
