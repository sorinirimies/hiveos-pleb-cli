use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};

// Account
#[derive(Debug, Args)]
pub struct AccountCommand {
    #[clap(subcommand)]
    pub command: AccountSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AccountSubcommand {
    /// Authenticates a given HiveOS account and returns a Bearer token
    AuthLogin(AccountAuth),

    /// Saves a user account in the local DB
    Save(UserAccount),

    /// Update an existing user in the local DB
    Update(UserAccount),

    /// Delete a user account based on the given username if found from the local DB
    Delete(DeleteAccount),

    /// Shows an account based on the given username from the local DB
    Show(ShowAccount),
}

#[derive(Debug, Args)]
pub struct AccountAuth {
    pub username: String,

    pub password: String,

    pub twofa_code: String,
}

#[derive(Debug, PartialEq, Args, Deserialize, Serialize)]
pub struct UserAccount {
    /// The email of user
    pub email: String,

    /// The username
    pub username: String,

    /// The access token for the account
    pub account_token: String,
}

#[derive(Debug, Args, Deserialize)]
pub struct DeleteAccount {
    /// The username provided in order to delete the account
    pub username: String,
}

#[derive(Debug, Args)]
pub struct ShowAccount {
    /// Show the account based on username
    pub username: String,
}