use crate::parser::Node;

#[derive(Debug)]
pub(crate) struct Query {
    path: Vec<QueryItem>,
}

#[derive(Debug)]
struct QueryItem {
    title: String,
    attr_key: Option<String>,
    text: bool,
}


impl Query {
    pub fn from(q: &str) -> Self {
        let v: Vec<_> = q
            .split(">")
            .map(|n| {
                if let Some((title, param)) = n.split_once(":") {
                    if param.contains("text") {
                        QueryItem { title: title.to_string(), text: true, attr_key: None }
                    } else {
                        let attr_name = param.strip_prefix("attr[")
                            .and_then(|a| a.strip_suffix("]"))
                            .expect(&*format!("expected: 'attr[attr_name]' but got: {}", &param));
                        QueryItem { title: title.to_string(), text: false, attr_key: Some(attr_name.to_string()) }
                    }
                } else {
                    QueryItem { title: n.to_string(), text: false, attr_key: None }
                }
            })
            .collect();

        Self { path: v }
    }

    pub fn search(&self, nodes: &[Node], depth: usize) {
        for n in nodes {
            if depth < self.path.len() {
                if n.name != self.path[depth].title {
                    continue;
                }
                self.search(&n.nodes, depth + 1);
            }

            if depth == self.path.len() - 1 {
                self.path[depth].print_node_if_match(n);
            }
        }
    }
}

impl QueryItem {
    fn print_node_if_match(&self, n: &Node) {
        if !self.title.eq(&n.name) {
            return;
        }

        if self.text {
            if let Some(t) = &n.text {
                println!("{t}");
            } else {
                println!();
            }
            return;
        }

        if let Some(key) = &self.attr_key {
            for a in &n.attr {
                if a.key.eq(key) {
                    println!("{}", a.val);
                    return;
                }
            }
        }

        println!("{}", n.name);
    }
}
