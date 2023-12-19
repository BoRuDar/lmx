#[derive(Default)]
pub struct Document {
    pub nodes: Vec<Node>,
}

#[derive(Default)]
pub struct Node {
    pub name: String,
    pub attr: Vec<Attr>,
    pub text: Option<String>,
    pub nodes: Vec<Node>,
}

pub struct Attr {
    pub key: String,
    pub val: String,
}
