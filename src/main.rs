use clap::Parser;
use ops::overclock_ops::{self, handle_overclock_command};

use crate::args::{EntityType, HiveOsArgs};
use crate::ops::account_ops::handle_account_command;
use crate::ops::farm_ops::handle_farm_command;
use crate::ops::flight_sheet_ops::handle_flight_sheet_command;
use crate::ops::wallet_ops::handle_wallet_command;
use crate::ops::worker_ops::handle_worker_command;

mod args;
mod ops;

fn main() {
    let args = HiveOsArgs::parse();
    handle_hive_os_cli_args(args);
}

fn handle_hive_os_cli_args(args: HiveOsArgs) {
    match args.entity_type {
        EntityType::Account(account) => handle_account_command(account),
        EntityType::Farm(farm) => handle_farm_command(farm),
        EntityType::Worker(worker) => handle_worker_command(worker),
        EntityType::FlightSheet(flight_sheet) => handle_flight_sheet_command(flight_sheet),
        EntityType::Wallet(wallet) => handle_wallet_command(wallet),
        EntityType::Overclock(overclock) => handle_overclock_command(overclock),
    }
}
