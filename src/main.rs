#![allow(unused_imports, unused_variables, dead_code)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use ethers::utils::hex;
use fhe::bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey};
use fhe_account_handler::get_keys;
use fhe_account_handler::user::*;
use fhe_node::fhe_oracle::Oracle;
use fhe_traits::Serialize;
use fhe_traits::*;
use rand::rngs::OsRng;
use rocket_contrib::json::Json;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_helper::structs::*;
use std::sync::Arc;

mod rocket_helper {
    pub(crate) mod structs;
}

mod fhe_account_handler {
    pub(crate) mod get_keys;
    pub(crate) mod user;
}

mod fhe_node {
    pub(crate) mod fhe_execution;
    pub(crate) mod fhe_oracle;
}

static mut ORACLE: Option<Oracle> = None;
static mut USER: Option<User> = None;

#[get("/")]
fn index() -> Json<Message> {
    Json(Message {
        message: "Welcome to the FHE Node!",
    })
}

#[post("/create_account", format = "json", data = "<data>")]
fn create_account(data: Json<CreateAccount>) -> Json<OracleUserString> {
    unsafe {
        if ORACLE.is_none() {
            ORACLE = Some(Oracle::new());
        }

        let user: User = create_user(
            data.address.clone(),
            ORACLE.as_ref().unwrap().parameters.clone(),
            Some(data.der_key.clone()),
            Some(0),
        );

        USER = Some(user.clone());

        let user_oracle: OracleUserString = OracleUserString {
            address: user.address.clone(),
            fhe_pk: hex::encode(user.fhe_pk.to_bytes()),
            fhe_balance: hex::encode(user.fhe_balance.to_bytes()),
        };

        Json(user_oracle)
    }
}

fn make_cors() -> rocket_cors::Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        // Add your specific origins here. Note that `*` cannot be used
        // if you have the credentials flag enabled.
        "http://localhost:3000", // replace with your actual origin
    ]);

    CorsOptions {
        allowed_origins,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Cors configuration failed")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, create_account])
        .attach(make_cors())
        .launch();
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}
