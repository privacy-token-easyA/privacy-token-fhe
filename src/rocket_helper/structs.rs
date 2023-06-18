use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Message {
    pub message: &'static str,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DepositFunds {
    pub address: String,
    pub amount: String,
    pub der_key: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SendFunds {
    pub address_from: String,
    pub address_to: String,
    pub amount: String, // String?
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WithdrawFunds {
    pub address_to: String,
    pub amount: String, // String?
}

#[derive(Deserialize, Serialize, Clone)]
pub struct OracleUserString {
    pub address: String,
    pub fhe_pk: String,
    pub fhe_balance: String,
}
