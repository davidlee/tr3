use crate::data::Node;
use clap::Parser;
use commands::Commands::*;

pub(crate) mod commands;

pub(crate) mod ui {
    use crate::data::Node;

    pub fn print_nodes(nodes: Vec<Node>) -> () {
        println!("id | parent | descr ");
        println!("--------------------------------");
        for node in nodes {
            let p: String = node.parent_id.map_or("".to_string(), |x| x.to_string());
            println!("{:?}  | {}       | {}", node.id, p, node.descr);
        }
    }
}

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
        Add { parent_id, slop } => match Node::insert(ctx, parent_id, slop) {
            Ok(n) => println!("Created task {:?}.", n),
            Err(e) => println!("error: {:?}", e),
        },
        List => match Node::list(ctx) {
            Ok(nodes) => ui::print_nodes(nodes),
            Err(e) => println!("error: {:?}", e),
        },
        Modify { id: _, slop: _ } => stub(),
        Delete { id: _ } => stub(),
    }
}

pub fn stub() -> () {}
