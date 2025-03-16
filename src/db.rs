use resolve_path::PathResolveExt;
use rusqlite::{Connection, Result};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub(crate) mod schema;

pub fn connect() -> Result<Connection, rusqlite::Error> {
    ensure_database_exists();
    Connection::open(path())
}

fn data_base_dir() -> PathBuf {
    Path::new(&env::var("XDG_DATA_HOME").unwrap_or_else(|_| "~/.local/share".to_owned()))
        .resolve()
        .to_path_buf()
}

fn data_dir() -> PathBuf {
    data_base_dir().join("tr3")
}

fn path() -> PathBuf {
    data_dir().join("tr3.sqlite")
}

fn file_exists() -> bool {
    path().try_exists().is_ok_and(|x| !!x)
}

fn ensure_database_exists() {
    if !file_exists() {
        fs::create_dir_all(data_dir()).unwrap();
        let conn = Connection::open(path()).unwrap();
        schema::install(&conn).unwrap();
    }
}
