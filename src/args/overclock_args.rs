use clap::{Args, Subcommand};
/// Overclock Template Command
#[derive(Debug, Args)]
pub struct OverclockCommand {
    #[clap(subcommand)]
    pub command: OverclockSubcommand,
}

/// Overclock Template Subcommand
#[derive(Debug, Subcommand)]
pub enum OverclockSubcommand {
    Show(ShowOVerclockTemplates),
    Create(CreateOverclockTemplate),
}

#[derive(Debug, Args)]
pub struct ShowOVerclockTemplates {
    /// Shows the overclock templates for a given username
    pub username: String,
    /// Farm ID to show overclock templates
    pub farm_id: i64,
}

#[derive(Debug, Args)]
pub struct CreateOverclockTemplate {
    /// Creates an overclock template for a given username
    pub username: String,
    /// Farm ID to show overclock templates
    pub farm_id: i64,
    /// The given friendly name for the overclock template
    pub overclock_template_name: String,
}
