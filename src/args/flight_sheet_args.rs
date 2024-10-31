use clap::{Args, Subcommand};

/// Flight Sheet Command
#[derive(Debug, Args)]
pub struct FlightSheetCommand {
    #[clap(subcommand)]
    pub command: FlightSheetSubcommand,
}

/// Flight Sheet SubCommand
#[derive(Debug, Subcommand)]
pub enum FlightSheetSubcommand {
    Show(ShowFlightSheet)
}
#[derive(Debug, Args)]
pub struct ShowFlightSheet {
    /// Show the Flight Sheets for a given username
    pub username: String,
}