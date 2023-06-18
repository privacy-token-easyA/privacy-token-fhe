use ethers::abi::{decode, encode, Token};
use fhe::bfv::{Ciphertext, Plaintext, PublicKey, SecretKey};
use fhe_traits::Serialize;
use std::process::Output;
use std::str;

use crate::fhe_tx_sender::contract_deployer::get_deployed_address;

use tokio::io::AsyncReadExt;
use tokio::process::Command;

pub async fn deposit_tokens_tx_sender(
    pk: &PublicKey,
    priv_key: &String,
    fhe_balance: &Ciphertext,
    amount: &String,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let deployed_address = get_deployed_address();

    let pk_bytes = pk.to_bytes();
    let pk_encoded = Token::Bytes(pk_bytes).to_string();

    let fhe_balance_bytes = fhe_balance.to_bytes();
    let fhe_balance_encoded = Token::Bytes(fhe_balance_bytes).to_string();

    let output = Command::new("cast")
        .arg("send")
        .arg(deployed_address)
        .arg("deposit_fETH(string,string)")
        .arg(pk_encoded)
        .arg(fhe_balance_encoded)
        .arg("--private-key")
        .arg(priv_key)
        .arg("--value")
        .arg(amount)
        .output()
        .await?;

    match get_tx_hash(output).await {
        Ok(tx_hash) => Ok(tx_hash),
        Err(error) => {
            eprintln!("Failed to execute script: {}", error);
            Ok(None)
        }
    }
}

pub async fn send_fhe_tx(
    fhe_tx_sender: &str,
    fhe_tx_receiver: &str,
    fhe_proof: &str,
    priv_key: &String,
    amount: &String,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let deployed_address = get_deployed_address();

    let output = Command::new("cast")
        .arg("send")
        .arg(deployed_address)
        .arg("send_fhe_tx(string,string,string)")
        .arg(fhe_tx_sender)
        .arg(fhe_tx_receiver)
        .arg(fhe_proof)
        .arg("--private-key")
        .arg(priv_key)
        .arg("--value")
        .arg(amount)
        .output()
        .await?;

    match get_tx_hash(output).await {
        Ok(tx_hash) => Ok(tx_hash),
        Err(error) => {
            eprintln!("Failed to execute script: {}", error);
            Ok(None)
        }
    }
}

pub async fn withdraw_ETH_request(
    amount: &String,
    fhe_sk: &SecretKey,
    fhe_new_pk: &PublicKey,
    fhe_balance: &Ciphertext,
    priv_key: &String,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let deployed_address = get_deployed_address();

    let sk_bytes = fhe_sk.to_bytes().to_vec();
    let sk_encoded = Token::Bytes(sk_bytes).to_string();

    let pk_bytes = fhe_new_pk.to_bytes();
    let pk_new_encoded = Token::Bytes(pk_bytes).to_string();

    let fhe_balance_bytes = fhe_balance.to_bytes();
    let fhe_balance_encoded = Token::Bytes(fhe_balance_bytes).to_string();

    let output = Command::new("cast")
        .arg("send")
        .arg(deployed_address)
        .arg("withdraw_ETH_request(uint256,string,string,string)")
        .arg(amount)
        .arg(sk_encoded)
        .arg(pk_new_encoded)
        .arg(fhe_balance_encoded)
        .arg("--private-key")
        .arg(priv_key)
        .arg("--value")
        .arg(amount)
        .output()
        .await?;

    match get_tx_hash(output).await {
        Ok(tx_hash) => Ok(tx_hash),
        Err(error) => {
            eprintln!("Failed to execute script: {}", error);
            Ok(None)
        }
    }
}

pub async fn withdraw_ETH_confirm(
    amount: &String,
    recv_address: &String,
    priv_key: &String,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let output = Command::new("cast")
        .arg("send")
        .arg(recv_address)
        .arg("--value")
        .arg(amount)
        .arg("--private-key")
        .arg(priv_key)
        .output()
        .await?;

    match get_tx_hash(output).await {
        Ok(tx_hash) => Ok(tx_hash),
        Err(error) => {
            eprintln!("Failed to execute script: {}", error);
            Ok(None)
        }
    }
}

async fn get_tx_hash(output: Output) -> Result<Option<String>, Box<dyn std::error::Error>> {
    if output.status.success() {
        let mut stdout = String::new();
        tokio::io::AsyncReadExt::read_to_string(&mut &output.stdout[..], &mut stdout).await?;

        let tx_hash = stdout
            .split("transactionHash")
            .nth(1)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split(",")
            .nth(0)
            .unwrap()
            .trim()
            .trim_matches('\"')
            .to_string();

        Ok(Some(tx_hash))
    } else {
        eprintln!("Error: {:?}", output.stderr);
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fhe::bfv::{BfvParameters, Encoding, SecretKey};
    use fhe_traits::{FheEncoder, FheEncrypter};
    use rand::thread_rng;
    use std::sync::Arc;

    use crate::fhe_node::fhe_oracle::OracleUser;
    use crate::fhe_node::{fhe_execution::Tx, fhe_oracle::Oracle};
    use crate::{
        fhe_account_handler::{
            get_keys::{get_keys, tests::create_users},
            user::{create_user, User},
        },
        fhe_tx_sender::contract_deployer::FEE,
    };
    #[tokio::test]
    async fn test_deposit_fETH() {
        let rng = thread_rng();

        let (fhe_oracle, alice, bob, owner) = create_users(100, 50);

        let priv_key = get_keys("owner").unwrap().private_key.to_string();
        let pk = owner.fhe_pk.clone();
        let fhe_balance = owner.fhe_balance.clone();

        let tx_hash =
            deposit_tokens_tx_sender(&pk, &priv_key, &fhe_balance, &FEE.to_string()).await;
        assert!(tx_hash.is_ok());
    }

    #[tokio::test]
    async fn test_send_fhe_tx() {
        let rng = thread_rng();

        let (fhe_oracle, alice, bob, owner) = create_users(100, 50);

        let priv_key = get_keys("owner").unwrap().private_key.to_string();
        let pk = owner.fhe_pk.clone();
        let fhe_balance = owner.fhe_balance.clone();

        let tx_hash =
            deposit_tokens_tx_sender(&pk, &priv_key, &fhe_balance, &FEE.to_string()).await;
        let bob_as_oracleuser: OracleUser = OracleUser::from_user(bob.clone());

        let tx = alice.create_tx(bob_as_oracleuser.clone(), &fhe_oracle, 10);

        let (tx_sender, tx_receiver) = tx.serialize_ct_tx_string();

        let tx_hash = send_fhe_tx(
            &tx_sender,
            &tx_receiver,
            &tx.tx_proof,
            &priv_key,
            &FEE.to_string(),
        )
        .await;

        assert!(tx_hash.is_ok());
    }

    #[tokio::test]
    async fn test_withdraw_ETH_request() {
        let rng = thread_rng();

        let (fhe_oracle, alice, bob, owner) = create_users(100, 50);

        let priv_key = get_keys("owner").unwrap().private_key.to_string();
        let fhe_sk = owner.fhe_sk.clone();
        let fhe_pk = alice.fhe_pk.clone();
        let fhe_balance = owner.fhe_balance.clone();

        let tx_hash =
            deposit_tokens_tx_sender(&fhe_pk, &priv_key, &fhe_balance, &FEE.to_string()).await;

        let tx_hash =
            withdraw_ETH_request(&FEE.to_string(), &fhe_sk, &fhe_pk, &fhe_balance, &priv_key).await;

        assert!(tx_hash.is_ok());
    }
}
