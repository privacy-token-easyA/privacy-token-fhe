pub struct KeyPair {
    pub public_key: &'static str,
    pub private_key: &'static str,
}

impl KeyPair {
    pub fn new(public_key: &'static str, private_key: &'static str) -> Self {
        Self {
            public_key,
            private_key,
        }
    }
}

pub fn get_keys(user: &str) -> Option<KeyPair> {
    match user {
        "owner" => Some(KeyPair::new(
            "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
        )),
        "alice" => Some(KeyPair::new(
            "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
            "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        )),
        "bob" => Some(KeyPair::new(
            "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC",
            "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a",
        )),
        "charlie" => Some(KeyPair::new(
            "0x90F79bf6EB2c4f870365E785982E1f101E93b906",
            "0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6",
        )),
        "dave" => Some(KeyPair::new(
            "0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65",
            "0x47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a",
        )),
        _ => None,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    // creates alice and bob, adds then to the oracle and returns the oracle and returns them
    use super::*;
    use crate::fhe_account_handler::user::{create_user, test::test_create_users, User};
    use crate::fhe_node::fhe_oracle::{Oracle, OracleUser};

    pub fn create_users(alice_balance: u64, bob_balance: u64) -> (Oracle, User, User, User) {
        let mut fhe_oracle = Oracle::new();

        let owner = create_user(
            get_keys("owner").unwrap().public_key.to_string(),
            fhe_oracle.parameters.clone(),
            None,
            Some(100),
        );

        let alice = create_user(
            get_keys("alice").unwrap().public_key.to_string(),
            fhe_oracle.parameters.clone(),
            None,
            Some(alice_balance),
        );

        let bob = create_user(
            get_keys("bob").unwrap().public_key.to_string(),
            fhe_oracle.parameters.clone(),
            None,
            Some(bob_balance),
        );

        fhe_oracle.add_user(owner.address.clone(), OracleUser::from_user(owner.clone()));
        fhe_oracle.add_user(alice.address.clone(), OracleUser::from_user(alice.clone()));
        fhe_oracle.add_user(bob.address.clone(), OracleUser::from_user(bob.clone()));

        (fhe_oracle, alice, bob, owner)
    }
}
