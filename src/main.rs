#![allow(unused_imports, unused_variables, dead_code)]

use fhe::bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey};
use fhe_traits::*;
use rand::{rngs::OsRng, thread_rng};
use std::sync::Arc;

mod fhe_account_handler {
    pub(crate) mod get_keys;
    pub(crate) mod user;
}

mod fhe_node {
    pub(crate) mod fhe_execution;
    pub(crate) mod fhe_oracle;
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}
