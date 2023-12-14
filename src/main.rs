use std::fs::File;
use std::io::{Read};
use std::str::from_utf8;

fn main() -> std::io::Result<()> {
    let file = File::open("./files/test1.xml")?;
    reader(file);

    Ok(())
}

fn reader<R>(mut reader: R) -> Option<Node>
    where R: Read {
    let mut buf: [u8; 32] = [0; 32];

    let mut cur_node: Option<Node> = None;
    let mut nodes = Vec::new();

    let mut literal = Vec::new();
    let mut prev_tok: Token = Token::Space;

    // let mut tokens = Vec::new();
    let mut operations = Vec::new();
    let mut literals = Vec::new();

    while let Ok(n) = reader.read(&mut buf) {
        for b in &buf[..n] {
            match b {
                b'<' => {
                    if let Some(cur) = cur_node {
                        nodes.push(cur);
                    }
                    cur_node = Some(Node::new());
                    operations.push(Token::LAngle);
                    prev_tok = Token::LAngle;
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => {
                    literal.push(*b);
                    prev_tok = Token::Char;
                }
                b'>' => {
                    prev_tok = Token::RAngle;
                    while let Some(op) = operations.pop() {
                        match op {
                            Token::Eq => {
                                let val = literals.pop().expect("expected attr=val");
                                let key = literals.pop().expect("expected attr=val");
                                if let Some(cur) = &mut cur_node {
                                    cur.add_attr(Attr { key, val });
                                }
                            }
                            Token::LAngle => {
                                if let Some(cur) = &mut cur_node {
                                    cur.set_name(literals.pop().expect("expected Node name"));
                                }
                                break;
                            }
                            _ => { panic!("unreachable code") }
                        }
                    }
                }
                b' ' | b'\n' => {
                    if prev_tok == Token::Char {
                        let l = from_utf8(&literal).unwrap();
                        literals.push(l.to_string());
                        literal = Vec::new();
                    }
                    prev_tok = Token::Space;
                }
                b'=' => {
                    let l = from_utf8(&literal).unwrap();
                    literals.push(l.to_string());
                    literal = Vec::new();
                    operations.push(Token::Eq);
                    prev_tok = Token::Eq;
                }
                b'/' => {
                    prev_tok = Token::FSlash;
                }
                b'"' => {
                    if prev_tok == Token::Char {
                        let l = from_utf8(&literal).unwrap();
                        literals.push(l.to_string());
                        literal = Vec::new();
                    }
                    prev_tok = Token::Quote;
                }
                _ => { panic!("unknown token: {}", b.to_string()) }
            }
        }

        if n < buf.len() {
            break;
        }
    }

    cur_node
}

#[derive(Debug, PartialEq)]
enum Token {
    ID(String),
    Attr(String),
    LAngle,
    RAngle,
    Eq,
    FSlash,
    Char,
    Space,
    Quote,
}

#[derive(Debug, PartialEq)]
struct Node {
    name: Option<String>,
    val: Option<String>,
    attr: Option<Vec<Attr>>,
    elems: Option<Vec<Box<Node>>>,
}

impl Node {
    fn new() -> Self {
        Self { name: None, val: None, attr: None, elems: None }
    }

    fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
    fn set_val(&mut self, val: String) {
        self.val = Some(val);
    }
    fn add_attr(&mut self, attr: Attr) {
        if let Some(a) = &mut self.attr {
            a.push(attr);
        } else {
            self.attr = Some(vec![attr]);
        }
    }
}

#[derive(Debug, PartialEq)]
struct Attr {
    key: String,
    val: String,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test1() {
        let s = "<title attr=\"Name\"/>".as_bytes();
        let want = Node {
            name: Some("title".to_string()),
            val: None,
            elems: None,
            attr: Some(vec![Attr { key: "attr".to_string(), val: "Name".to_string() }]),
        };
        let res = reader(Cursor::new(s));
        assert_eq!(Some(want), res);
    }
}