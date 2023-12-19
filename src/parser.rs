use std::collections::VecDeque;
use crate::lexer::Token;
use crate::parts::{Attr, Document, Node};

pub struct Parser {
    tokens: VecDeque<Token>,
    cur_tok: Token,
    next_tok: Token,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Option<Self> {
        let mut tokens = VecDeque::from(tokens.to_vec());
        let cur_tok = tokens.pop_front()?;
        let next_tok = tokens.pop_front()?;
        Some(Self { cur_tok, next_tok, tokens })
    }

    fn move_to_next(&mut self) -> Option<Token> {
        let ret = self.cur_tok.to_owned();
        self.cur_tok = self.next_tok.to_owned();
        self.next_tok = self.tokens.pop_front()?;
        Some(ret)
    }

    pub fn parse(&mut self) -> Document {
        let mut doc = Document::default();

        loop {
            match &self.cur_tok {
                Token::EOF => break,
                Token::OpenNode(name) => {
                    if let Some(n) = self.parse_node(name.to_owned()) {
                        doc.nodes.push(n);
                    }
                }
                t => panic!("unexpected token: {t:?}")
            }
        }

        doc
    }

    fn parse_node(&mut self, node_name: String) -> Option<Node> {
        self.move_to_next();
        let mut node = Node { name: node_name, ..Node::default() };

        // parse attributes if any
        while let Some(attr) = self.parse_attr() {
            node.attr.push(attr);
        }

        loop {
            match self.cur_tok.to_owned() {
                Token::CloseNode => {
                    self.move_to_next();
                    return Some(node);
                }

                Token::CloseNodeNamed(name) => {
                    if node.name != name { panic!("open<{}> and close<{name}> blocks don't match", node.name) }
                    self.move_to_next();
                    return Some(node);
                }

                Token::OpenNode(embedded_node_name) => {
                    if let Some(n) = self.parse_node(embedded_node_name) {
                        node.nodes.push(n);
                    }
                }

                Token::EndOfOpenNode => { self.move_to_next(); }

                Token::String(inner_text) => {
                    let mut tmp_str = vec![inner_text];
                    self.move_to_next();

                    while let Token::String(s) = self.cur_tok.to_owned() {
                        tmp_str.push(s);
                        self.move_to_next();
                    }
                    node.text = Some(tmp_str.join(" "));
                }
                t => panic!("unexpected token: {t:?}")
            }
        }
    }

    fn parse_attr(&mut self) -> Option<Attr> {
        let Token::String(key) = self.cur_tok.to_owned() else { return None; };
        self.move_to_next();

        if self.cur_tok != Token::Eq { panic!("must be '=' in 'attr=\"val\"") }
        self.move_to_next();

        let Token::Quote(val) = self.cur_tok.to_owned() else { panic!("attr val must be a quoted string") };
        self.move_to_next();

        Some(Attr { key, val })
    }
}
