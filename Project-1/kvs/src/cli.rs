use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Opt {
    #[command(subcommand)]
    pub command: Option<Commands>,
}
#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// Set key
    Set { key: String, val: String },
    /// Get key
    Get { key: String },
    /// Remove key
    Rm { key: String },
}
