use ureq::Error;

use crate::args::overclock_args::{OverclockCommand, OverclockSubcommand};
use crate::ops::account_ops::get_account_bearer_token;

pub fn handle_overclock_command(overclock: OverclockCommand) {
    let command = overclock.command;

    match command {
        OverclockSubcommand::Show(show_overclock) => {
            let bearer_token_result = get_account_bearer_token(show_overclock.username);
            let account_token = bearer_token_result.unwrap().account_token;
            let result = get_overclock_templates(account_token, show_overclock.farm_id);
            print!("Farm response is: {}", result.unwrap());
        }
        OverclockSubcommand::Create(create_overclock) => {
            let bearer_token_result = get_account_bearer_token(create_overclock.username);
            let account_token = bearer_token_result.unwrap().account_token;
            let result = create_overclock_template(account_token, create_overclock.farm_id);
            print!("Farm response is: {}", result.unwrap());
        }
    }
}

pub fn get_overclock_templates(bearer_token: String, farm_id: i64) -> Result<String, Error> {
    let url = format!("https://api2.hiveos.farm/api/v2/farms/{}/oc", farm_id);

    let response = ureq::get(&url)
        .set("Authorization", &format!("Bearer {}", bearer_token))
        .set("Content-Type", "application/json")
        .call()?
        .into_string()?;

    Ok(response)
}

pub fn create_overclock_template(bearer_token: String, farm_id: i64) -> Result<String, Error> {
    let url = format!("https://api2.hiveos.farm/api/v2/farms/{}/oc", farm_id);

    let response = ureq::post(&url)
        .set("Authorization", &format!("Bearer {}", bearer_token))
        .set("Content-Type", "application/json")
        .send_string(
            r#"{
            "name": "My Overclock Template",
            "type": "GPU",
            "info": {
                "algo": "ethash",
                "coin": "ETH"
            },
            "options": {
                "force_enable": true
            },
            "items": [{
                "worker_ids": [],
                "options": {
                    "nvidia": {
                        "power_limit": 130,
                        "core_clock": 100,
                        "memory_clock": 1000,
                        "fan_speed": 80
                    },
                    "amd": {
                        "core_clock": 1200,
                        "memory_clock": 2100,
                        "core_voltage": 850,
                        "memory_voltage": 850,
                        "fan_speed": 80
                    }
                }
            }]
        }"#,
        )?
        .into_string()?;

    Ok(response)
}
