use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Node {
    id: i32,
    parent_id: Option<i32>,
    descr: String,
}

#[derive(Debug)]
struct Tag {
    id: i32,
    parent_id: Option<i32>,
    name: String,
}

#[derive(Debug)]
struct Tagging {
    tag_id: i32,
    node_id: i32,
}

fn main() -> Result<()> {
    println!("Hello, world!");

    let conn = Connection::open_in_memory()?;

    conn.execute("PRAGMA foreign_keys = ON", ())?;
    conn.execute(
        "
        CREATE TABLE Node (
            id INTEGER PRIMARY KEY,
            parent_id INTEGER,
            descr TEXT NOT NULL,
            
            FOREIGN KEY(parent_id) REFERENCES Node(id)
        )",
        (),
    )?;
    conn.execute(
        "
        CREATE TABLE Tag (
            id INTEGER PRIMARY KEY,
            parent_id INTEGER,
            name TEXT NOT NULL,   

            FOREIGN KEY(parent_id) REFERENCES Tag(id)
        )",
        (),
    )?;
    conn.execute(
        "
        CREATE TABLE Tagging (
            tag_id INTEGER NOT NULL,
            node_id INTEGER NOT NULL,

            FOREIGN KEY(node_id) REFERENCES Node(id)
            FOREIGN KEY(tag_id) REFERENCES Tag(id)
        )",
        (),
    )?;
    Ok(())
}
