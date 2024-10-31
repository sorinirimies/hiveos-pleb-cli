use ureq::Error;

use crate::args::farm_args::{CreateFarm, FarmCommand, FarmSubcommand, RemoveFarm};
use crate::ops::account_ops::get_account_bearer_token;

pub fn handle_farm_command(farm: FarmCommand) {
    let command = farm.command;

    match command {
        FarmSubcommand::Show(show_farm) => {
            let bearer_token_result = get_account_bearer_token(show_farm.username);
            let account_token = bearer_token_result.unwrap().account_token;
            let result = get_farms(account_token);
            print!("Farm response is: {}", result.unwrap());
        }
        FarmSubcommand::Create(create_farm) => {
            let bearer_token_result = get_account_bearer_token(create_farm.username.clone());
            let account_token = bearer_token_result.unwrap().account_token;

            match create_new_farm(account_token, &create_farm) {
                // Added & here
                Ok(response) => println!("Farm created successfully: {}", response),
                Err(e) => eprintln!("Failed to create farm: {}", e),
            }
        }
        FarmSubcommand::Remove(remove_farm) => {
            let bearer_token_result = get_account_bearer_token(remove_farm.username.clone());
            let account_token = bearer_token_result.unwrap().account_token;
            match delete_farm(account_token, &remove_farm) {
                Ok(response) => println!("Farm removed successfully: {}", response),
                Err(e) => eprintln!("Failed to remove farm: {}", e),
            }
        }
    }
}

fn get_farms(bearer_token: String) -> Result<String, Error> {
    let header_token = format!("Bearer {}", &bearer_token);

    let resp: String = ureq::get("https://api2.hiveos.farm/api/v2/hive/miners")
        .set("Authorization", &header_token)
        .set("Content-Type", "application/json")
        .call()?
        .into_string()?;

    Ok(resp)
}

fn create_new_farm(bearer_token: String, create_farm: &CreateFarm) -> Result<String, Error> {
    // Changed parameter to &CreateFarm
    let header_token = format!("Bearer {}", &bearer_token);

    let resp: String = ureq::post("https://api2.hiveos.farm/api/v2/farms")
        .set("Authorization", &header_token)
        .set("Content-Type", "application/json")
        .send_json(ureq::json!({
            "name": &create_farm.farm_name,
            "auto_tags": create_farm.auto_tags,
            "auto_updates": create_farm.auto_updates
        }))?
        .into_string()?;

    Ok(resp)
}

fn delete_farm(
    bearer_token: String,
    remove_farm: &RemoveFarm,
) -> Result<String, Box<dyn std::error::Error>> {
    let header_token = format!("Bearer {}", &bearer_token);

    // First, get the farm ID if farm_name is provided instead of ID
    let farms_response = get_farms(bearer_token.clone())?;

    // Parse the response to find the farm ID
    let farms_value: ureq::serde_json::Value = ureq::serde_json::from_str(&farms_response)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // Find the farm ID by name
    let farm_id = farms_value
        .as_array()
        .and_then(|farms| {
            farms
                .iter()
                .find(|farm| farm["name"].as_str() == Some(&remove_farm.farm_name))
                .and_then(|farm| farm["id"].as_u64())
        })
        .ok_or_else(|| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Farm not found",
            ))
        })?;

    // If force flag is not set, ask for confirmation
    if !remove_farm.force {
        println!("Warning: This will delete the farm and all associated data.");
        println!("Please type 'YES' to confirm: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim() != "YES" {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Operation cancelled by user",
            )));
        }
    }

    // Delete the farm
    let delete_url = format!("https://api2.hiveos.farm/api/v2/farms/{}", farm_id);
    let resp: String = ureq::delete(&delete_url)
        .set("Authorization", &header_token)
        .set("Content-Type", "application/json")
        .call()?
        .into_string()?;

    Ok(resp)
}
