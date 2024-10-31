use clap::{Args, Parser, Subcommand};

use crate::args::account_args::AccountCommand;
use crate::args::farm_args::FarmCommand;
use crate::args::flight_sheet_args::FlightSheetCommand;
use crate::args::wallet_args::WalletCommand;
use crate::args::worker_args::WorkerCommand;

pub mod wallet_args;
pub mod account_args;
pub mod flight_sheet_args;
pub mod worker_args;
pub mod farm_args;

/// The HiveOS main tree arguments.
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct HiveOsArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// CRUD operations for a given HiveOS account.
    Account(AccountCommand),
    /// CRUD operations for a given HiveOS Farm.
    Farm(FarmCommand),
    /// CRUD operations for a given HiveOS Worker.
    Worker(WorkerCommand),
    /// CRUD operations for a given HiveOS FlightSheet.
    FlightSheet(FlightSheetCommand),
    /// CRUD operations for a given HiveOS Wallet.
    Wallet(WalletCommand),
}
