use crate::data::Node;
use clap::Parser;
use commands::Commands::*;
use rusqlite::Connection;

pub(crate) mod commands;

pub(crate) mod ui {
    use crate::data::Node;
    use comfy_table::{Table, presets::ASCII_FULL_CONDENSED}; // no lines between rows

    pub fn print_nodes(nodes: Vec<Node>) -> () {
        let mut table = Table::new();
        table.load_preset(ASCII_FULL_CONDENSED);
        table.set_header(vec!["id", "parent", "description"]);

        for node in nodes {
            let id = node.id.to_string();
            let parent = node.parent_id.map_or("".to_string(), |x| x.to_string());
            table.add_row(vec![id, parent, node.descr]);
        }
        println!("{table}");
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

pub fn dispatch(conn: &mut Connection) {
    let args = Args::parse();

    match args.cmd {
        Add { parent_id, slop } => match Node::insert(conn, parent_id, slop) {
            Ok(n) => println!("Created task {:?}.", n),
            Err(e) => println!("error: {:?}", e),
        },
        List => match Node::list(conn) {
            Ok(nodes) => ui::print_nodes(nodes),
            Err(e) => println!("error: {:?}", e),
        },
        Modify { id: _, slop: _ } => stub(),
        Delete { id: _ } => stub(),
    }
}

pub fn stub() -> () {}
