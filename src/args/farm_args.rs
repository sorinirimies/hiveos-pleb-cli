use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};

/// Hive OS Farm Command
#[derive(Debug, Args)]
pub struct FarmCommand {
    #[clap(subcommand)]
    pub command: FarmSubcommand,
}

/// Hive OS Farm Subcommand
#[derive(Debug, Subcommand)]
pub enum FarmSubcommand {
    /// Show farm details
    Show(ShowFarm),
    /// Create a new mining farm
    Create(CreateFarm),
    /// Remove an existing mining farm
    Remove(RemoveFarm),
}

#[derive(Debug, Args)]
pub struct ShowFarm {
    /// Show the farms for a given username
    pub username: String,
}

#[derive(Debug, PartialEq, Args, Deserialize, Serialize)]
pub struct CreateFarm {
    /// Username of the farm owner
    pub username: String,
    /// Name of the mining farm
    pub farm_name: String,
    /// Enable auto tags for workers
    #[clap(long)]
    pub auto_tags: bool,
    /// Enable auto updates for workers
    #[clap(long)]
    pub auto_updates: bool,
}

#[derive(Debug, Args)]
pub struct RemoveFarm {
    /// Username of the farm owner
    pub username: String,
    /// Name or ID of the farm to remove
    pub farm_name: String,
    /// Force removal without confirmation (WARNING: This will delete all associated workers and data)
    #[clap(short, long)]
    pub force: bool,
}
