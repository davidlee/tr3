use rusqlite::{Connection, Result};

// implements the Repository pattern
// maps object data to relational database operations on behalf of commands

#[derive(Debug, Clone)]
pub struct Node {
    pub id: i64, // TODO add unstable identifier
    pub parent_id: Option<i64>,
    pub descr: String,
}

impl Node {
    pub fn insert(conn: &mut Connection, parent_id: Option<i64>, slop: Vec<String>) -> Result<i64> {
        conn.execute(
            "INSERT INTO Node (parent_id, descr) VALUES (?1, ?2)",
            (parent_id, slop.join(" ")),
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn list(conn: &mut Connection) -> Result<Vec<Node>> {
        let mut stmt = conn.prepare("SELECT * FROM Node")?;
        let rows = stmt.query_map([], |row| {
            Ok(Node {
                id: row.get(0)?,
                parent_id: row.get(1)?,
                descr: row.get(2)?,
            })
        })?;
        let mut nodes = Vec::new();

        for node in rows {
            nodes.push(node?);
        }
        Ok(nodes)
    }
}

#[derive(Debug)]
struct Tag {
    id: i32,
    parent_id: Option<i32>,
    name: String,
    full_name: String,
}

impl Tag {}
