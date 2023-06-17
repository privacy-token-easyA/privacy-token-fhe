use crate::fhe_node::fhe_oracle::Oracle;
use ethers::utils::hex;
use fhe::bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey};
use fhe_traits::*;

#[derive(Clone)]
pub struct Tx {
    pub tx_hash: String,
    pub sender: String,
    pub receiver: String,
    pub tx_sender: Ciphertext,
    pub tx_receiver: Ciphertext,
    pub tx_proof: String,
}

impl Tx {
    pub fn new(
        tx_hash: String,
        sender: String,
        receiver: String,
        tx_sender: Ciphertext,
        tx_receiver: Ciphertext,
        tx_proof: String,
    ) -> Tx {
        Tx {
            tx_hash,
            sender,
            receiver,
            tx_sender,
            tx_receiver,
            tx_proof,
        }
    }

    pub fn decode_from_onchain_tx(
        fhe_oracle: &mut Oracle,
        tx_hash: String,
        sender: String,
        receiver: String,
        tx_sender: String,
        tx_receiver: String,
        tx_proof: String,
    ) -> Tx {
        let sender = hex::encode(sender.as_bytes());

        let fhe_tx_sender = hex::decode(tx_sender).unwrap();
        let fhe_tx_receiver = hex::decode(tx_receiver).unwrap();

        let tx_sender =
            Ciphertext::from_bytes(&fhe_tx_sender, &fhe_oracle.parameters.clone()).unwrap();
        let tx_receiver =
            Ciphertext::from_bytes(&fhe_tx_receiver, &fhe_oracle.parameters.clone()).unwrap();

        Tx {
            tx_hash: hex::encode(tx_hash.as_bytes()),
            sender,
            receiver,
            tx_sender,
            tx_receiver,
            tx_proof,
        }
    }

    pub fn serialize_ct_tx_string(&self) -> (String, String) {
        let tx_sender = hex::encode(self.tx_sender.to_bytes());
        let tx_receiver = hex::encode(self.tx_receiver.to_bytes());

        (tx_sender, tx_receiver)
    }

    pub fn execute_tx(&self, fhe_oracle: &mut Oracle) -> Oracle {
        let tx = self.clone();

        // check if tx_hash is in LIST_OF_TXS
        check_tx_hash(tx.tx_hash.clone());

        unsafe {
            LIST_OF_TXS.push(tx.clone());
        }

        let sender = tx.sender.clone();
        let receiver = tx.receiver.clone();

        let sender_tx = tx.tx_sender.clone();
        let receiver_tx = tx.tx_receiver.clone();

        let sender_fhe_balance: Ciphertext = fhe_oracle.return_user_fhe_balance(sender.clone());
        let receiver_fhe_balance: Ciphertext = fhe_oracle.return_user_fhe_balance(receiver.clone());

        let sender_fhe_balance = &sender_fhe_balance - &sender_tx;
        let receiver_fhe_balance = &receiver_fhe_balance + &receiver_tx;

        fhe_oracle.update_user_fhe_balance(sender.clone(), sender_fhe_balance);
        fhe_oracle.update_user_fhe_balance(receiver.clone(), receiver_fhe_balance);

        fhe_oracle.clone()
    }
}

pub fn check_tx_hash(tx_hash: String) -> bool {
    let mut tx_exists = false;

    for tx in unsafe { LIST_OF_TXS.iter() } {
        if tx.tx_hash == tx_hash {
            tx_exists = true;
            println!("Tx already exists in LIST_OF_TXS");
            break;
        }
    }

    false
}

fn bytes_to_hex_string(bytes: &[u8]) -> String {
    let hex_chars: Vec<String> = bytes.iter().map(|byte| format!("{:02x}", byte)).collect();

    hex_chars.join("")
}

pub(crate) static mut LIST_OF_TXS: Vec<Tx> = Vec::new();
