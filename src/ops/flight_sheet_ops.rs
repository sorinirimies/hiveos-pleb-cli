use ureq::Error;

use crate::args::flight_sheet_args::{FlightSheetCommand, FlightSheetSubcommand};
use crate::ops::account_ops::get_account_bearer_token;

pub fn handle_flight_sheet_command(flight_sheet: FlightSheetCommand) {
    let command = flight_sheet.command;

    match command {
        FlightSheetSubcommand::Show(show_flight_sheet) => {
            let bearer_token_result = get_account_bearer_token(show_flight_sheet.username);
            let account_token = bearer_token_result.unwrap().account_token;
            let _result = get_flight_sheets(account_token);
            print!("Farm responses is: {}", _result.unwrap());
        }
    }
}

fn get_flight_sheets(bearer_token: String) -> Result<String, Error> {
    let header_token = format!("Bearer {}", &bearer_token);

    let resp: String = ureq::get("https://api2.hiveos.farm/api/v2/hive/miners")
        .set("Authorization", &header_token)
        .set("Content-Type", "application/json")
        .call()?
        .into_string()?;

    Ok(resp)
}


