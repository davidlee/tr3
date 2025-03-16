// implements the Repository pattern
// maps object data to relational database operations on behalf of commands

#[derive(Debug, Clone)]
pub struct Node {
    id: i32,
    parent_id: Option<i32>,
    descr: String,
}

impl Node {
    pub fn insert(ctx: &mut crate::AppContext, descr: String) -> crate::Result<bool> {
        let conn = &ctx.connection;
        let result = conn.execute("INSERT INTO Node (descr) VALUES (?1)", [descr]);
        match result {
            Ok(1) => return Ok(true),
            Ok(_) => return Ok(false),
            Err(e) => return Err(crate::Error::Rusqlite(e)),
        }
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
