use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Account,
    Transaction,
    Loan,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChannelType {
    Email,
    SMS,
}

#[derive(Debug, Deserialize)]
pub struct AccountData {
    pub event_id: String,
    pub user_id: String,
    pub event_type: EventType,
    pub channel: ChannelType,
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
    pub channel: ChannelType,
    pub email: String,
    pub phone: String,
    pub loan_id: String,
    pub loan_amount: String,
    pub status: String,
    pub message: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct TransactionData {
    pub event_id: String,
    pub user_id: String,
    pub event_type: EventType,
    pub channel: ChannelType,
    pub email: String,
    pub phone: String,
    pub transaction_id: String,
    pub amount: String,
    pub transaction_type: String,
    pub balance: String,
    pub message: String,
    pub created_at: String,
}
