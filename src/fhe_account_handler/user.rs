use crate::{
    fhe_account_handler::get_keys::get_keys,
    fhe_node::{fhe_execution::Tx, fhe_oracle::*},
};

use fhe::bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey};
use fhe_traits::*;
use rand::{rngs::OsRng, thread_rng};
use std::sync::Arc;

#[derive(Clone)]
pub struct User {
    pub address: String,
    pub key_path: String,
    pub der_key: String,
    pub fhe_sk: SecretKey,
    pub fhe_pk: PublicKey,
    pub fhe_balance: Ciphertext,
}

impl User {
    pub fn new(
        address: String,
        key_path: String,
        der_key: String,
        fhe_sk: SecretKey,
        fhe_pk: PublicKey,
        fhe_balance: Ciphertext,
    ) -> User {
        User {
            address,
            key_path,
            der_key,
            fhe_sk,
            fhe_pk,
            fhe_balance,
        }
    }

    pub fn create_tx(&self, receiver: OracleUser, oracle: &Oracle, value: u64) -> Tx {
        let sender = self.clone();

        let mut rng = thread_rng();

        assert!(sender.user_balance(oracle) >= value, "Insufficient funds");
        assert!(value > 0, "Value must be greater than 0");

        let fhe_value =
            Plaintext::try_encode(&[value], Encoding::poly(), &oracle.parameters).unwrap();

        Tx::new(
            String::new(),
            self.address.clone(),
            receiver.address.clone(),
            sender.fhe_pk.try_encrypt(&fhe_value, &mut rng).unwrap(),
            receiver.fhe_pk.try_encrypt(&fhe_value, &mut rng).unwrap(),
            String::new(),
        )
    }

    pub fn user_balance(&self, oracle: &Oracle) -> u64 {
        let oracle_user = oracle.users.get(&self.address).unwrap();
        let decrypted_plaintext = self.fhe_sk.try_decrypt(&oracle_user.fhe_balance).unwrap();
        let decrypted_vector =
            Vec::<u64>::try_decode(&decrypted_plaintext, Encoding::poly()).unwrap();

        decrypted_vector[0]
    }
}

pub fn decoded_user_balance(user: &User) -> u64 {
    let decrypted_plaintext = user.fhe_sk.try_decrypt(&user.fhe_balance).unwrap();
    let decrypted_vector = Vec::<u64>::try_decode(&decrypted_plaintext, Encoding::poly()).unwrap();

    decrypted_vector[0]
}
pub fn create_user(
    address: String,
    parameters: Arc<fhe::bfv::BfvParameters>,
    der_key: Option<String>,
    start_balance: Option<u64>,
) -> User {
    let mut rng = thread_rng();

    let der_key = der_key.unwrap_or("default".to_string());
    let start_balance = start_balance.unwrap_or(0);

    let mut key_path = "keys/".to_string() + &address;
    let sk = SecretKey::random_and_write_to_file(&parameters, &mut OsRng, &mut key_path);

    let pk = PublicKey::new(&sk, &mut rng);

    let balance: Plaintext =
        Plaintext::try_encode(&[start_balance], Encoding::poly(), &parameters).unwrap();
    let fhe_balance: Ciphertext = sk.try_encrypt(&balance, &mut rng).unwrap();

    User::new(address, key_path, der_key, sk, pk, fhe_balance).clone()
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::fhe_account_handler::get_keys::tests::create_users;
    #[test]
    pub fn test_create_users() {
        let init_alice_balance = 100;
        let init_bob_balance = 50;

        let (fhe_oracle, alice, bob, owner) = create_users(100, 50);

        assert!(
            decoded_user_balance(&alice) == init_alice_balance,
            "Alice's balance is incorrect"
        );
        assert!(
            decoded_user_balance(&bob) == init_bob_balance,
            "Bob's balance is incorrect"
        );
    }

    #[test]
    fn test_tx_send_and_receive() {
        let init_alice_balance = 100;
        let init_bob_balance = 50;
        let delta_balance = 10;

        let (fhe_oracle, alice, bob, owner) = create_users(100, 50);

        let bob_user: OracleUser = fhe_oracle.users[&bob.address].clone();

        let txs = alice.create_tx(bob_user, &fhe_oracle, delta_balance);

        let fhe_oracle = txs.execute_tx(&mut fhe_oracle.clone());

        let alice_oracle = fhe_oracle.users[&alice.address].clone();
        let bob_oracle = fhe_oracle.users[&bob.address].clone();

        let alice = User {
            fhe_balance: alice_oracle.fhe_balance,
            ..alice
        };

        let bob = User {
            fhe_balance: bob_oracle.fhe_balance,
            ..bob
        };

        assert!(
            decoded_user_balance(&alice) == init_alice_balance - delta_balance,
            "Alice's balance is incorrect"
        );

        assert!(
            decoded_user_balance(&bob) == init_bob_balance + delta_balance,
            "Bob's balance is incorrect"
        );
    }

    #[test]
    fn test_tx_withdraw() {
        let init_alice_balance = 100;
        let delta_balance = 20;

        let (mut fhe_oracle, alice, bob, owner) = create_users(100, 50);

        let new_alice = create_user(
            alice.address.clone(),
            fhe_oracle.parameters.clone(),
            None,
            Some(0),
        );

        let new_alice_oracle_user: OracleUser = OracleUser::from_user(new_alice.clone());

        let txs = alice.create_tx(new_alice_oracle_user.clone(), &fhe_oracle, 80);

        let sk = alice.fhe_sk.clone();
        let new_pk = new_alice_oracle_user.fhe_pk.clone();
        let new_fhe_balance: Ciphertext = new_alice.fhe_balance.clone();

        fhe_oracle = txs.execute_withdrawal(
            &mut fhe_oracle,
            sk,
            delta_balance,
            new_pk.clone(),
            new_fhe_balance,
        );

        //assert the new fhe_oracle has the updated pk for alice
        assert_eq!(
            fhe_oracle.return_user_pk(new_alice.address.clone()),
            new_pk.clone()
        );

        let new_alice = User {
            fhe_balance: txs.tx_receiver.clone(),
            ..new_alice
        };
        let new_user_balance = decoded_user_balance(&new_alice.clone());

        assert_eq!(new_user_balance, init_alice_balance - delta_balance)
    }
}
