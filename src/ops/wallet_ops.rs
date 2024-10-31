use ureq::Error;

use crate::args::wallet_args::{WalletCommand, WalletSubcommand};
use crate::ops::account_ops::get_account_bearer_token;

pub fn handle_wallet_command(wallet: WalletCommand) {
    let wallet_command = wallet.command;
    match wallet_command {
        WalletSubcommand::Show(wallet_command) => {
            let bearer_token_result = get_account_bearer_token(wallet_command.username);
            let account_token = bearer_token_result.unwrap().account_token;
            let result = get_wallets(account_token);
            println!("Wallets are: {:?}", result.unwrap());
        }
    }
}


fn get_wallets(bearer_token: String) -> Result<String, Error> {
    let header_token = format!("Bearer {}", &bearer_token);

    let resp: String = ureq::get("https://api2.hiveos.farm/api/v2/wallets")
        .set("Authorization", &header_token)
        .set("Content-Type", "application/json")
        .call()?
        .into_string()?;

    Ok(resp)
}


