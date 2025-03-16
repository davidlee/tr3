pub(crate) mod cli;
pub(crate) mod db;
pub(crate) mod objects;

#[derive(Debug, Clone)]
enum Error {
    ArgumentError,
    IoError,
    ValidationError,
}

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    db::connect().unwrap();
    let _result = cli::dispatch();
    Ok(())
}
