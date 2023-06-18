#![allow(unused_imports, unused_variables, dead_code)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use ethers::utils::hex;
use fhe::bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey};
use fhe_account_handler::get_keys;
use fhe_account_handler::user::*;
use fhe_node::fhe_oracle::Oracle;
use fhe_node::fhe_oracle::OracleUser;
use fhe_traits::Serialize;
use fhe_traits::*;
use fhe_tx_sender::tx_sender;
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

mod fhe_tx_sender {
    pub(crate) mod contract_deployer;
    pub(crate) mod tx_sender;
}

static mut ORACLE: Option<Oracle> = None;
static mut USER: Option<User> = None;

#[get("/")]
fn index() -> Json<MessageApi> {
    Json(MessageApi {
        message: "Welcome to the FHE Node!",
    })
}

#[post("/deposit_funds", format = "json", data = "<data>")]
fn deposit_funds(
    data: Json<OracleUserApi>,
) -> Result<Json<ResponseApi>, Box<dyn std::error::Error>> {
    unsafe {
        if ORACLE.is_none() {
            ORACLE = Some(Oracle::new());
        }
        let mut toAdd = 0;
        if USER.is_some(){
            toAdd = USER.as_ref().unwrap().user_balance(&ORACLE.as_mut().unwrap());
        }
        
        let user: User = create_user(
            data.sender_address.clone(),
            ORACLE.as_ref().unwrap().parameters.clone(),
            Some(data.der_key.clone()),
            // TODO make balance add onto itself
            Some(data.amount.clone().parse::<u64>().unwrap()+toAdd),
        );

        USER = Some(user.clone());

        let user_as_oracle_user: OracleUser = OracleUser {
            address: user.address.clone(),
            fhe_pk: user.fhe_pk.clone(),
            fhe_balance: user.fhe_balance.clone(),
        };

        ORACLE
            .as_mut()
            .unwrap()
            .add_user(data.sender_address.clone(), user_as_oracle_user.clone());
        let priv_key = get_keys::get_keys("user").unwrap().private_key.to_string();
        let pk = user.fhe_pk.clone();
        let fhe_balance = user.fhe_balance.clone();

        let result = tokio::runtime::Runtime::new().unwrap().block_on(async {
            let tx_hash =
                tx_sender::deposit_tokens_tx_sender(&pk, &priv_key, &fhe_balance, &data.amount)
                    .await;

            let tx_hash = tx_hash.unwrap();
            let response: ResponseApi = ResponseApi {
                res: tx_hash.unwrap(),
                res_status: "Success".to_string(),
            };
            USER = Some(user.clone());
            Json(response)
        });

        Ok(result)
    }
}

#[post("/send_funds", format = "json", data = "<data>")]
fn send_funds(data: Json<OracleUserApi>) -> Result<Json<ResponseApi>, Box<dyn std::error::Error>> {
    unsafe {
        //println!("108 data: {:?}", data);
        if ORACLE.is_none() {
            return Ok(Json(ResponseApi {
                res: "Deposit first".to_string(),
                res_status: "Error".to_string(),
            }));
        }
        //println!("115 data: {:?}", data);

        let user: User = USER.as_ref().unwrap().clone();

        //println!("119 data: {:?}", data);
        let user_as_oracle_user: OracleUser = OracleUser {
            address: user.address.clone(),
            fhe_pk: user.fhe_pk.clone(),
            fhe_balance: user.fhe_balance.clone(),
        };
        //println!("125 data: {:?}", data);

        let oracle = ORACLE.as_mut().unwrap().clone();
        let receiver_fhe_balance = oracle.return_user_fhe_balance(data.receiver_address.clone());
        let receiver_fhe_pk = oracle.return_user_pk(data.receiver_address.clone());

        //println!("131 data: {:?}", data);
        let receiver_as_oracle_user: OracleUser = OracleUser {
            address: data.receiver_address.clone(),
            fhe_pk: receiver_fhe_pk.clone(),
            fhe_balance: receiver_fhe_balance.clone(),
        };

        //println!("138 data: {:?}", data);
        let user_pk = get_keys::get_keys("user").unwrap().private_key;

        let tx = user.create_tx(
            receiver_as_oracle_user.clone(),
            &ORACLE.as_mut().unwrap().clone(),
            data.amount.parse::<u64>().unwrap(),
        );

        let (tx_sender, tx_receiver) = tx.serialize_ct_tx_string();

        let result = tokio::runtime::Runtime::new().unwrap().block_on(async {
            let tx_hash = tx_sender::send_fhe_tx(
                &tx_sender,
                &tx_receiver,
                &"0".to_string().as_str(),
                &user_pk.to_string(),
                &data.amount.clone(),
            )
            .await;
            let tx_hash = tx_hash.unwrap();
            let response: ResponseApi = ResponseApi {
                res: tx_hash.unwrap(),
                res_status: "Success".to_string(),
            };
            USER = Some(user.clone());
            Json(response)
        });

        Ok(result)
    }
}

#[post("/withdraw_funds", format = "json", data = "<data>")]
fn withdraw_funds(
    data: Json<OracleUserApi>,
) -> Result<Json<ResponseApi>, Box<dyn std::error::Error>> {
    unsafe {
        if ORACLE.is_none() {
            return Ok(Json(ResponseApi {
                res: "Deposit first".to_string(),
                res_status: "Error".to_string(),
            }));
        }

        let user: User = USER.as_ref().unwrap().clone();

        let user_balance = user.user_balance(&ORACLE.as_mut().unwrap());

        let user_new: User = create_user(
            data.sender_address.clone(),
            ORACLE.as_ref().unwrap().parameters.clone(),
            Some(data.der_key.clone()),
            Some(0),
        );

        let user_as_oracle_user: OracleUser = OracleUser {
            address: user.address.clone(),
            fhe_pk: user.fhe_pk.clone(),
            fhe_balance: user.fhe_balance.clone(),
        };

        let new_user_as_oracle_user: OracleUser = OracleUser {
            address: user_new.address.clone(),
            fhe_pk: user_new.fhe_pk.clone(),
            fhe_balance: user_new.fhe_balance.clone(),
        };

        let oracle = ORACLE.as_mut().unwrap().clone();
        let receiver_fhe_balance = oracle.return_user_fhe_balance(data.receiver_address.clone());
        let receiver_fhe_pk = oracle.return_user_pk(data.receiver_address.clone());

        let user_pk = get_keys::get_keys("user").unwrap().private_key;

        let tx = user.create_tx(
            user_as_oracle_user.clone(),
            &ORACLE.as_mut().unwrap(),
            user_balance - data.amount.parse::<u64>().unwrap(),
        );

        let (tx_sender, tx_receiver) = tx.serialize_ct_tx_string();
        let user_fhe_sk = user.fhe_sk.clone();
        let result = tokio::runtime::Runtime::new().unwrap().block_on(async {
            let tx_hash = tx_sender::withdraw_ETH_request(
                &data.amount.clone(),
                &user.fhe_sk.clone(),
                &new_user_as_oracle_user.fhe_pk.clone(),
                &new_user_as_oracle_user.fhe_balance.clone(),
                &user_pk.to_string(),
            )
            .await;
            let tx_hash = tx_hash.unwrap();
            let response: ResponseApi = ResponseApi {
                res: tx_hash.unwrap(),
                res_status: "Success".to_string(),
            };
            let tx_hash = tx_sender::withdraw_ETH_confirm(
                &data.amount.clone(),
                &data.sender_address.clone(),
                &get_keys::get_keys("user").unwrap().private_key.to_string(),
            )
            .await;
            Json(response)
        });

        Ok(result)
    }
}

#[get("/get_balance")]
fn get_balance() -> Result<Json<ResponseApi>, Box<dyn std::error::Error>> {
    unsafe {
        if ORACLE.is_none() {
            return Ok(Json(ResponseApi {
                res: "Deposit first".to_string(),
                res_status: "Error".to_string(),
            }));
        }
        let user: User = USER.as_ref().unwrap().clone();

        let user_balance = user.user_balance(&ORACLE.as_mut().unwrap()).to_string();

        let response: ResponseApi = ResponseApi {
            res: user_balance,
            res_status: "Success".to_string(),
        };

        Ok(Json(response))
    }
}

fn make_cors() -> rocket_cors::Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        // Add your specific origins here. Note that `*` cannot be used
        // if you have the credentials flag enabled.
        "http://localhost:3000", // replace with your actual origin
        "http://localhost:8000",
        "http://localhost:5173",
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
        .mount(
            "/",
            routes![
                index,
                deposit_funds,
                send_funds,
                withdraw_funds,
                get_balance
            ],
        )
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
