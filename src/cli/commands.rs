use clap::Subcommand;

pub mod add;
pub mod delete;
pub mod done;
pub mod get_config;
pub mod list;
pub mod modify;
pub mod set_config;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Add { slop: Vec<String> },
    Modify { id: String, descr: String },
    List,
    Done { id: String },
    Delete { id: String },
    GetConfig { key: String },
    SetConfig { key: String, value: String },
}
