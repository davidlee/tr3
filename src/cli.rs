use crate::data::Node;
use clap::Parser;
use commands::Commands::*;

pub(crate) mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]

struct Args {
    #[command(subcommand)]
    cmd: commands::Commands,

    #[arg(last = true)]
    slop: Vec<String>,
}

pub fn dispatch(ctx: &mut crate::AppContext) {
    let args = Args::parse();

    match args.cmd {
        Add { slop } => match Node::insert(ctx, slop) {
            Ok(n) => println!("Ok, {:?}", n),
            Err(e) => println!("error: {:?}", e),
        },
        List => match Node::list(ctx) {
            Ok(nodes) => {
                for node in nodes {
                    println!("Node: {:?}", node);
                }
            }
            Err(_) => println!("error."),
        },
        Modify { id: _, descr: _ } => stub(),
        Done { id: _ } => stub(),
        Delete { id: _ } => stub(),
        GetConfig { key: _ } => stub(),
        SetConfig { key: _, value: _ } => stub(),
    }
}

pub fn stub() -> () {}
