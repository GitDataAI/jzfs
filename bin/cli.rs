use crate::api::api;
use clap::{Parser, Subcommand};
use error::AppError;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand, about = "show or create config file")]
    Config(ConfigCommand),
    #[command(about = "Run the application")]
    Run,
    #[command(about = "Show the application status")]
    Status,
    #[command(about = "Restart the application")]
    Restart,
    #[command(about = "Stop the application")]
    Stop,
    #[command(about = "Show application log")]
    Log,
    #[command(about = "Migrate database")]
    Migration,
}

pub mod migration;

#[derive(Subcommand)]
pub enum ConfigCommand {
    #[command(about = "Initialize a new config file")]
    Init {
        #[arg(short, long, default_value = "./config.toml")]
        path: Option<String>,
    },
    #[command(about = "Show the current config")]
    Show {
        #[arg(short, long)]
        all: bool,
        #[arg(short, long)]
        git: bool,
        #[arg(short, long)]
        database: bool,
        #[arg(short, long)]
        redis: bool,
        #[arg(short, long)]
        api: bool,
    },
}

pub async fn cli() -> Result<(), AppError> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Config(_config) => {}
        Commands::Run => {
            println!("Starting...");
            api().await?;
        }
        Commands::Status => {}
        Commands::Restart => {}
        Commands::Stop => {}
        Commands::Log => {}
        Commands::Migration => migration::migration().await,
    }
    Ok(())
}
