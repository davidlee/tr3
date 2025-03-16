use clap::{Parser, Subcommand};
use rusqlite::Result;

#[derive(Parser)]
#[command(version, about)]

struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Add { descr: String },
    Modify { id: String, descr: String },
    List,
    Done { id: String },
    Delete { id: String },
    ListConfig,
    GetConfig { key: String },
    SetConfig { key: String, value: String },
}

pub fn dispatch() -> Result<()> {
    let args = Args::parse();

    match args.cmd {
        Commands::Add { descr: _ } => {}
        Commands::Modify { id: _, descr: _ } => {}
        Commands::List => {}
        Commands::Done { id: _ } => {}
        Commands::Delete { id: _ } => {}
        Commands::ListConfig => {}
        Commands::GetConfig { key: _ } => {}
        Commands::SetConfig { key: _, value: _ } => {}
    }

    Ok(())
}
