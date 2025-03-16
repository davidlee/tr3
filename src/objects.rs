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
