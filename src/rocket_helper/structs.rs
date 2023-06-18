use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Message {
    pub message: &'static str,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CreateAccount {
    pub address: String,
    pub der_key: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct OracleUserString {
    pub address: String,
    pub fhe_pk: String,
    pub fhe_balance: String,
}
