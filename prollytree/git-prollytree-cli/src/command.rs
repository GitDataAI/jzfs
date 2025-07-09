use clap::{Parser, Subcommand};

#[derive(Debug,Parser)]
#[command(name = "git-data")]
#[command(about = "A command line tool for managing git data")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug,Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Install {
        #[arg(
            short,
            long,
            default_value = ".",
            help = "The directory to install git-data into"
        )]
        dir: String,
    },
    #[command(arg_required_else_help = true)]
    Clone {
        #[arg(
            short,
            long,
            default_value = ".",
            help = "The directory to clone into"
        )]
        dir: String,
    },
    #[command(arg_required_else_help = true)]
    Push,
    #[command(arg_required_else_help = true)]
    Pull,
    #[command(arg_required_else_help = true)]
    Migrate,
    #[command(arg_required_else_help = true)]
    Track,
    #[command(arg_required_else_help = true)]
    UnTrack,
    #[command(arg_required_else_help = true)]
    LsFile,
}