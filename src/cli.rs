use clap::Parser;

pub(crate) mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]

struct Args {
    #[command(subcommand)]
    cmd: commands::Commands,

    #[arg(last = true)]
    slop: Vec<String>,
}

pub fn dispatch(ctx: &mut crate::AppContext) -> crate::Result<bool> {
    let args = Args::parse();

    match args.cmd {
        commands::Commands::Add { slop } => crate::data::Node::insert(ctx, slop),
        commands::Commands::Modify { id: _, descr: _ } => stub(),
        commands::Commands::List => stub(),
        commands::Commands::Done { id: _ } => stub(),
        commands::Commands::Delete { id: _ } => stub(),
        commands::Commands::ListConfig => stub(),
        commands::Commands::GetConfig { key: _ } => stub(),
        commands::Commands::SetConfig { key: _, value: _ } => stub(),
    }
}

pub fn stub() -> Result<bool, crate::Error> {
    Err(crate::Error::NotImplementedError)
}
