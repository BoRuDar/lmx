use std::fmt::Formatter;

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

impl std::fmt::Display for Attr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.val)
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("<{}", self.name);

        for a in &self.attr {
            s = format!("{} {}=\"{}\"", s, a.key, a.val);
        }

        s = format!("{}>", s);

        if let Some(text) = &self.text {
            s = format!("{}{}", s, text);
        } else {
            s = format!("{}\n", s);
            for n in &self.nodes {
                s = format!("{}{}", s, n);
            }
        }

        write!(f, "{}</{}>\n", s, self.name)
    }
}

impl std::fmt::Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = "".to_string();

        for n in &self.nodes {
            s = format!("{}{}", s, n);
        }

        write!(f, "{}", s)
    }
}
