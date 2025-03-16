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
        #[arg(short = 'p', long = "parent")]
        parent_id: Option<i64>,

        #[arg(short = 't', long = "tag", value_delimiter=',', num_args=1..)]
        tags: Vec<String>,

        slop: Vec<String>, // descr
    },
    Modify {
        id: i64,

        #[arg(short = 't', long = "tag", value_delimiter=',', num_args=1..)]
        tags: Vec<String>,

        slop: Vec<String>,
    },
    List,
    Delete {
        id: i64,
    },
}
