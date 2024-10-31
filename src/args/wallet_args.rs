use clap::{Args, Subcommand};

/// Wallet Command
#[derive(Debug, Args)]
pub struct WalletCommand {
    #[clap(subcommand)]
    pub command: WalletSubcommand,
}

/// Wallet Subcommand
#[derive(Debug, Subcommand)]
pub enum WalletSubcommand {
    Show(ShowWallet)
}

#[derive(Debug, Args)]
pub struct ShowWallet {
    /// Show the wallets for a given username
    pub username: String,
}