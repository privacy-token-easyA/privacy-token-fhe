use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct MessageApi {
    pub message: &'static str,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct OracleUserApi {
    pub amount: String,
    pub sender_address: String,
    pub receiver_address: String,
    pub der_key: String,
    pub fhe_pk: String,
    pub fhe_balance: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ResponseApi {
    pub res: String,
    pub res_status: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DepositFundsApi {
    pub address: String,
    pub amount: String,
    pub fhe_pk: String,
    pub fhe_balance: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SendFundsApi {
    pub fhe_tx_sender: String,
    pub fhe_tx_receiver: String,
    pub fhe_proof: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WithdrawFundsApi {
    pub amount: String,
    pub fhe_sk: String,
    pub fhe_pk_new: String,
    pub fhe_balance_new: String,
}
