use rusqlite::{Connection, Result};

pub fn install(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        BEGIN;
        PRAGMA foreign_keys = ON;
            
        CREATE TABLE Nodes (
            id INTEGER PRIMARY KEY,
            parent_id INTEGER,
            descr TEXT NOT NULL,
            
            FOREIGN KEY(parent_id) REFERENCES Nodes(id)
        );

        CREATE TABLE Tags (
            id INTEGER PRIMARY KEY,
            parent_id INTEGER,
            name TEXT NOT NULL,   
            full_name TEXT NOT NULL, 

            FOREIGN KEY(parent_id) REFERENCES Tags(id)
        );

        CREATE UNIQUE INDEX uniq_tag_name ON Tags (name);
        CREATE UNIQUE INDEX uniq_tag_full_name ON Tags (full_name);

        CREATE TABLE Taggings (
            tag_id INTEGER NOT NULL,
            node_id INTEGER NOT NULL,

            FOREIGN KEY(node_id) REFERENCES Nodes(id)
            FOREIGN KEY(tag_id) REFERENCES Tags(id)
        );

        CREATE UNIQUE INDEX uniq_tagging ON Taggings (tag_id, node_id);
        COMMIT;
        ",
    )?;
    Ok(())
}
