use crate::fhe_account_handler::get_keys::{get_keys, KeyPair};
use std::process::Command;
use std::str;

// store the deployed address so that we can use it in other tests
static mut DEPLOYED_ADDRESS: Option<String> = None;
pub const DEPLOYED_BLOCK: u64 = 0;
pub const URL: &str = "http://127.0.0.1:8545";
pub const FEE: &str = "100";

// executes forge create src/contracts/fheETH.sol:FHEToken --constructor-args 8 100 --unlocked --from 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
fn deployer(owner: &str) -> Option<String> {
    let output = Command::new("forge")
        .arg("create")
        .arg("src/contracts/fheETH.sol:FHEToken")
        .arg("--constructor-args")
        .arg("8")
        .arg(FEE)
        .arg("--unlocked")
        .arg("--from")
        .arg(owner)
        .output()
        .expect("Failed to execute script");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).unwrap();
        let deployed_to_line = stdout.lines().find(|line| line.starts_with("Deployed to:"));
        if let Some(deployed_to_line) = deployed_to_line {
            let deployed_to = deployed_to_line
                .split(":")
                .nth(1)
                .map(|address| address.trim().to_string())
                .unwrap();

            unsafe {
                DEPLOYED_ADDRESS = Some(deployed_to.clone());
            }

            return Some(deployed_to);
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Script execution failed:\n{}", stderr);
    }

    None
}

pub fn get_deployed_address() -> &'static str {
    unsafe {
        if DEPLOYED_ADDRESS.is_none() {
            // The contract isn't deployed yet, so deploy it
            let owner = get_keys("owner").unwrap();
            deployer(owner.public_key);
        }
        DEPLOYED_ADDRESS.as_ref().map(|s| s.as_str()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployer() {
        unsafe {
            assert!(DEPLOYED_ADDRESS.is_none());
            get_deployed_address();
            assert!(DEPLOYED_ADDRESS.is_some());
        }
    }
}
