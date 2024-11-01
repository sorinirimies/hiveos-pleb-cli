use clap::{Args, Subcommand};

// Worker
#[derive(Debug, Args)]
pub struct WorkerCommand {
    #[clap(subcommand)]
    pub command: WorkerSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum WorkerSubcommand {
    Show(ShowWorker),
}
#[derive(Debug, Args)]
pub struct ShowWorker {
    /// Show the worker for a given username
    pub username: String,
}
