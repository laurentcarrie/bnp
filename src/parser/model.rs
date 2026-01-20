use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum SoldeType {
    Credit,
    Debit,
}

#[derive(Debug, Clone, Serialize)]
pub struct Operation {
    pub date: NaiveDate,
    pub nature_des_operations: String,
    pub valeur: NaiveDate,
    pub montant: f64,
    pub montant_type: SoldeType,
}

#[derive(Debug, Clone, Serialize)]
pub struct Solde {
    pub solde_type: SoldeType,
    pub montant: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Releve {
    pub date_du_releve: NaiveDate,
    pub solde_ouverture: Solde,
    pub solde_cloture: Solde,
    pub total_des_operations_debit: f64,
    pub total_des_operations_credit: f64,
    pub check_debit: f64,
    pub check_credit: f64,
    pub operations: Vec<Operation>,
}
