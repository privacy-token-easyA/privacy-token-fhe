#![allow(unused_imports, unused_variables, dead_code)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use fhe::bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey};
use fhe_traits::*;
use rand::{rngs::OsRng, thread_rng};
use std::sync::Arc;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_contrib::json::Json;
use serde::Serialize;

mod fhe_account_handler {
    pub(crate) mod get_keys;
    pub(crate) mod user;
}

mod fhe_node {
    pub(crate) mod fhe_execution;
    pub(crate) mod fhe_oracle;
}

// Import the cors crate
// rocket is for the webserver
#[derive(Serialize)]
struct Message {
    message: &'static str,
}

#[get("/")]
fn index() ->  Json<Message> {
    Json(Message {
        message: "Welcome to the FHE Node!",
    })
}
#[get("/testinput/<input>")]
fn testinput(input: String) -> String {
    input
}
#[get("/greet/<name>")]
fn greet(name: String) -> String {
    format!("Hey {}, glad to have you here!", name)
}

#[get("/check_tx_hash/<tx_hash>")]
fn call_check_tx_hash(tx_hash: String) -> String {
    fhe_node::fhe_execution::check_tx_hash(tx_hash).to_string()
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
        .mount("/", routes![index,greet,testinput,call_check_tx_hash])
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
