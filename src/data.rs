use regex::Regex;
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
    pub fn insert(
        conn: &mut Connection,
        parent_id: Option<i64>,
        tags: Vec<String>,
        slop: Vec<String>,
    ) -> Result<i64> {
        //
        if !tags.iter().all(|x| Tag::is_valid(x)) {
            // FIXME this is a hack, but we want to be able to use the execute()? idiom
            // figure out best approach to surface validation errors
            return Err(rusqlite::Error::InvalidQuery);
        }

        // create node
        conn.execute("BEGIN TRANSACTION;", ())?;
        conn.execute(
            "INSERT INTO Nodes (parent_id, descr) VALUES (?1, ?2)",
            (parent_id, slop.join(" ")),
        )?;
        let node_id = conn.last_insert_rowid();

        // add tags
        tags.iter().for_each(|x| {
            let tag_id = Tag::ensure(conn, &x);
            conn.execute(
                "INSERT INTO Taggings (node_id, tag_id) VALUES (?1, ?2)",
                (node_id, tag_id),
            )
            .unwrap();
        });

        conn.execute("COMMIT;", ())?; // TODO verify txn abort on failure

        Ok(node_id)
    }

    pub fn list(conn: &mut Connection) -> Result<Vec<Node>> {
        let mut stmt = conn.prepare("SELECT * FROM Nodes")?;
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

impl Tag {
    fn name_from_full_name(full_name: &str) -> &str {
        let v: Vec<_> = full_name.split('.').collect();
        v.last().unwrap()
    }

    fn is_valid(full_name: &str) -> bool {
        let rx = Regex::new(r"^([a-zA-Z0-9]+\.)*([a-zA-Z0-9]+)$").unwrap();
        return rx.is_match(full_name);
    }

    fn ensure(conn: &mut Connection, full_name: &str) -> i64 {
        let name = Tag::name_from_full_name(full_name);

        conn.execute(
            "INSERT INTO Tags (name, full_name) VALUES (?1, ?2)",
            (name, full_name),
        )
        .unwrap_or_default();

        let id: i64 = conn
            .query_row(
                "SELECT id FROM Tags WHERE full_name = ?1",
                [full_name],
                |row| row.get(0),
            )
            .unwrap();

        return id;
    }
}
