#![allow(dead_code, unused_imports)]

mod lazy_reader;
mod lexer;

use std::collections::{VecDeque};
use lazy_reader::LazyReader;
use lexer::Lexer;
use std::fs::File;
use crate::lexer::Token;

fn main() {
    let Ok(file) = File::open("./files/test3.xml") else {
        panic!("fail to open the file");
    };
    let lr = LazyReader::new(Box::new(file), 32);
    // let Some(mut p) = Parser::new(Lexer::new(lr).parse()) else { panic!("todo") };


    // dbg!(p.parse());
}

struct Parser {
    tokens: VecDeque<Token>,
    cur_tok: Token,
    next_tok: Token,
}

// impl Parser {
//     fn new(tokens: Vec<Token>) -> Option<Self> {
//         let mut tokens = VecDeque::from(tokens);
//         let cur_tok = tokens.pop_front()?;
//         let next_tok = tokens.pop_front()?;
//         Some(Self { cur_tok, next_tok, tokens })
//     }
//
//     fn next(&mut self) -> Option<Token> {
//         let ret = self.cur_tok.to_owned();
//         self.cur_tok = self.next_tok.to_owned();
//         self.next_tok = self.tokens.pop_front()?;
//         Some(ret)
//     }
//
//     pub fn parse(&mut self) -> Document {
//         let mut doc = Document { nodes: Vec::new() };
//
//         loop {
//             match &self.cur_tok {
//                 Token::EOF => break,
//                 Token::OpenNode => {
//                     if let Some(n) = self.parse_node() {
//                         doc.nodes.push(n);
//                     }
//                 }
//                 t => { println!("not implemented for {t:?}") }
//             }
//         }
//
//         doc
//     }
//
//     fn parse_node(&mut self) -> Option<Node> {
//         if self.cur_tok != Token::OpenNode {
//             return None; // return from recursive call
//         }
//
//         if self.cur_tok == Token::OpenNode && self.next_tok == Token::FSlash {
//             // reached  the end of the outer node at </node_name>
//             return None; // return from recursive call
//         }
//
//         self.next();
//         let Token::String(node_name) = self.cur_tok.to_owned() else { panic!("expected string!") };
//         self.next();
//
//         let mut node = Node { name: node_name, attr: vec![], node: vec![], text: None };
//
//         // parse attributes if any
//         while let Some(attr) = self.parse_attr() {
//             node.attr.push(attr);
//         }
//
//         // short block with like <block />
//         if self.cur_tok == Token::FSlash && self.next_tok == Token::RArrow {
//             self.next();
//             self.next();
//             return Some(node);
//         }
//
//         // end of opening block at '>'
//         if self.cur_tok == Token::RArrow {
//             self.next();
//
//             // embedded node
//             if self.cur_tok == Token::OpenNode {
//                 while let Some(internal_node) = self.parse_node() {
//                     node.node.push(internal_node);
//                 }
//             }
//
//             // inner text
//             if let Token::String(inner_text) = self.cur_tok.to_owned() {
//                 node.text = Some(inner_text);
//                 self.next();
//                 self.parse_closing_block(&node.name);
//                 return Some(node);
//             }
//         }
//
//
//         if self.parse_closing_block(&node.name) {
//             return Some(node);
//         }
//
//         if self.cur_tok != Token::FSlash || self.next_tok != Token::RArrow {
//             panic!("expected closing '/>'")
//         }
//
//         self.next();
//         self.next();
//         Some(node)
//     }
//
//     fn parse_closing_block(&mut self, name: &str) -> bool {
//         if self.cur_tok != Token::OpenNode && self.next_tok != Token::FSlash {
//             return false;
//         }
//         self.next();
//         self.next();
//         if let Token::String(block_name) = self.cur_tok.to_owned() {
//             if block_name != name {
//                 panic!("expected closing block name '</block_name>' must match the opening one '<block_name>'")
//             }
//         }
//         self.next();
//         if self.cur_tok != Token::RArrow {
//             panic!("expected closing '>' after the block's name")
//         }
//         self.next();
//
//         return true;
//     }
//
//     fn parse_attr(&mut self) -> Option<Attr> {
//         let Token::String(key) = self.cur_tok.to_owned() else { return None; };
//         self.next();
//
//         if self.cur_tok != Token::Eq {
//             panic!("must be '='")
//         }
//         self.next();
//
//         let Some(val) = self.parse_val()  else { panic!("attr val must be a quoted string") };
//
//         Some(Attr { key, val })
//     }
//
//     fn parse_val(&mut self) -> Option<String> {
//         if self.cur_tok != Token::Quote {
//             return None;
//         }
//         self.next();
//
//         if self.next_tok != Token::Quote {
//             return None;
//         }
//
//         let Token::String(val) = self.cur_tok.to_owned() else { panic!("attr val must be a string") };
//
//         self.next();
//         self.next();
//         Some(val)
//     }
// }

#[derive(Debug)]
struct Document {
    nodes: Vec<Node>,
}

#[derive(Debug)]
struct Node {
    name: String,
    attr: Vec<Attr>,
    node: Vec<Node>,
    text: Option<String>,
}

#[derive(Debug)]
struct Attr {
    key: String,
    val: String,
}

