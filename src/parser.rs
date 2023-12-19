use std::collections::VecDeque;
use std::fmt::{Formatter};
use crate::lexer::Token;

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

#[allow(dead_code)]
pub struct Attr {
    pub key: String,
    pub val: String,
}

pub struct Parser {
    tokens: VecDeque<Token>,
    cur_tok: Token,
    next_tok: Token,
}

impl Parser {
    pub(crate) fn new(tokens: &[Token]) -> Option<Self> {
        let mut tokens = VecDeque::from(tokens.to_vec());
        let cur_tok = tokens.pop_front()?;
        let next_tok = tokens.pop_front()?;
        Some(Self { cur_tok, next_tok, tokens })
    }

    fn next(&mut self) -> Option<Token> {
        let ret = self.cur_tok.to_owned();
        self.cur_tok = self.next_tok.to_owned();
        self.next_tok = self.tokens.pop_front()?;
        Some(ret)
    }

    pub fn parse(&mut self) -> Document {
        let mut doc = Document { nodes: Vec::new() };

        loop {
            match &self.cur_tok {
                Token::EOF => break,
                Token::OpenNode(name) => {
                    if let Some(n) = self.parse_node(name.to_owned()) {
                        doc.nodes.push(n);
                    }
                }
                t => {
                    panic!("not implemented for {t:?}");
                }
            }
        }

        doc
    }

    fn parse_node(&mut self, node_name: String) -> Option<Node> {
        self.next();
        let mut node = Node { name: node_name, ..Node::default() };

        // parse attributes if any
        while let Some(attr) = self.parse_attr() {
            node.attr.push(attr);
        }

        loop {
            match self.cur_tok.to_owned() {
                Token::CloseNode => {
                    self.next();
                    return Some(node);
                }
                Token::CloseNodeNamed(name) => {
                    if node.name != name {
                        panic!("open<{}> and close<{name}> blocks don't match", node.name);
                    }
                    self.next();
                    return Some(node);
                }
                Token::OpenNode(embedded_node_name) => {
                    if let Some(n) = self.parse_node(embedded_node_name) {
                        node.nodes.push(n);
                    }
                }
                Token::EndOfOpenNode => {
                    self.next();
                }
                Token::String(inner_text) => {
                    let mut tmp_str = Vec::new();
                    tmp_str.push(inner_text);
                    self.next();

                    while let Token::String(s) = self.cur_tok.to_owned() {
                        tmp_str.push(s);
                        self.next();
                    }
                    node.text = Some(tmp_str.join(" "));
                }
                tt => {
                    panic!("Unhandled case: {tt:?}");
                }
            }
        }
    }

    fn parse_attr(&mut self) -> Option<Attr> {
        let Token::String(key) = self.cur_tok.to_owned() else { return None; };
        self.next();

        if self.cur_tok != Token::Eq {
            panic!("must be '='")
        }
        self.next();

        let Token::Quote(val) = self.cur_tok.to_owned() else { panic!("attr val must be a quoted string") };
        self.next();

        Some(Attr { key, val })
    }
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
        }

        for n in &self.nodes {
            s = format!("{}\n{}", s, n);
        }

        write!(f, "{}</{}>\n", s, self.name)
    }
}

impl std::fmt::Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = "".to_string();

        for n in &self.nodes {
            s = format!("{} {}", s, n);
        }

        write!(f, "{}", s)
    }
}