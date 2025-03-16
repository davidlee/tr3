// use rusqlite::{Connection, MappedRows};

// implements the Repository pattern
// maps object data to relational database operations on behalf of commands

#[derive(Debug, Clone)]
pub struct Node {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub descr: String,
}

impl Node {
    pub fn insert(ctx: &mut crate::AppContext, slop: Vec<String>) -> rusqlite::Result<i64> {
        let descr = slop.join(" ");
        let conn = &ctx.connection;
        conn.execute("INSERT INTO Node (descr) VALUES (?1)", [descr])?;
        Ok(conn.last_insert_rowid())
    }

    pub fn list(ctx: &mut crate::AppContext) -> rusqlite::Result<Vec<Node>> {
        let conn = &ctx.connection;
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
}

#[derive(Debug)]
struct Tagging {
    tag_id: i32,
    node_id: i32,
}
