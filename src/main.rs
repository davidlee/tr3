pub(crate) mod cli;
pub(crate) mod data;
pub(crate) mod db;
pub(crate) mod objects;

#[derive(Debug)]
enum Error {
    ArgumentError,
    IoError,
    ValidationError,
    NotImplementedError,
    Rusqlite(rusqlite::Error),
}

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let mut conn = db::connect().unwrap();
    let _result = cli::dispatch(&mut conn);
    Ok(())
}

// struct AppContext {
//     connection: rusqlite::Connection,
// }
