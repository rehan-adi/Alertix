use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Account,
    Transaction,
    Loan,
}

#[derive(Debug, Deserialize)]
pub struct AccountData {
    pub event_id: String,
    pub user_id: String,
    pub event_type: EventType,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub message: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct LoanData {
    pub event_id: String,
    pub user_id: String,
    pub event_type: EventType,
    pub email: String,
    pub phone: String,
    pub loan_id: String,
    pub loan_amount: String,
    pub status: String,
    pub message: String,
    pub created_at: String,
}