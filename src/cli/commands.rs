use clap::Subcommand;

pub mod add;
pub mod delete;
pub mod done;
// pub mod get_config;
pub mod list;
pub mod modify;
// pub mod set_config;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Add {
        #[arg(short = 'p')]
        parent_id: Option<i64>,
        slop: Vec<String>,
    },
    Modify {
        id: i64,
        slop: Vec<String>,
    },
    List,
    // Done {
    //     id: i64,
    // },
    Delete {
        id: i64,
    },
}
