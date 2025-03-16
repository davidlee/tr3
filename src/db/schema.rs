use rusqlite::{Connection, Result};

// fn is_ready(conn: &Connection) -> bool {
//     if conn.execute("SELECT COUNT(*) FROM Node", ()).err() == None {
//         return true;
//     } else {
//         return false;
//     }
// }

pub fn install(conn: &Connection) -> Result<()> {
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
