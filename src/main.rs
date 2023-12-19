#![allow(dead_code, unused_variables)]

mod lazy_reader;
mod lexer;
mod parser;

use std::fs::File;
use clap::Parser;
use lazy_reader::LazyReader;
use lexer::Lexer;
use crate::parser::Node;

#[derive(clap::Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to file and its name: ./some/dirs/filename.xml
    #[arg(short, long)]
    filename: String,
    /// Query in format: node_name>sub>node:attr[name],text
    #[arg(short, long)]
    query: String,
}

fn main() {
    let args = Args::parse();

    let Ok(file) = File::open(&args.filename) else {
        panic!("fail to open the file");
    };
    let lr = LazyReader::new(Box::new(file), 32);
    let Some(mut p) = parser::Parser::new(Lexer::new(lr).parse().as_slice()) else { panic!("todo") };

    println!("{}", &args.query);
    search(&p.parse().nodes, &Query::from(&args.query).path, 0)
}


fn search(nodes: &[Node], path: &[QueryItem], depth: usize) {
    for n in nodes {
        if depth < path.len() {
            if n.name != path[depth].title {
                continue;
            }
            search(&n.nodes, &path, depth + 1);
        }
        if depth == path.len() - 1 {
            if let Some(t) = &n.text {
                println!("{t}");
            }
        }
    }
}

impl Query {
    fn from(q: &str) -> Self {
        let v: Vec<_> = q
            .split(">")
            .map(|n| {
                if let Some((title, params)) = n.split_once(":") {
                    QueryItem { title: title.to_string(), text: true }
                } else {
                    QueryItem { title: n.to_string(), text: false }
                }
            })
            .collect();

        Self { path: v }
    }
}

#[derive(Debug)]
struct Query {
    path: Vec<QueryItem>,
}

#[derive(Debug)]
struct QueryItem {
    title: String,
    text: bool,
}