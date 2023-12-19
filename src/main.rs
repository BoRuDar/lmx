mod lazy_reader;
mod lexer;
mod parser;
mod query;
mod parts;

use std::fs::File;
use clap::Parser;
use lazy_reader::LazyReader;
use lexer::Lexer;

#[derive(clap::Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to file and its name: ./some/dirs/filename.xml
    #[arg(short, long)]
    filename: String,
    /// Query format: node_name>sub_node>sub_node1
    /// | node_name>sub_node>sub_node1:attr[name]
    /// | node_name>sub_node>sub_node1:text
    #[arg(short, long)]
    query: String,
}

fn main() {
    let args = Args::parse();

    let Ok(file) = File::open(&args.filename) else {
        panic!("fail to open the file");
    };

    let lr = LazyReader::new(Box::new(file), 32);

    let Some(mut p) = parser::Parser::new(Lexer::new(lr).parse().as_slice()) else {
        panic!("parser init failed")
    };

    query::Query::from(&args.query).search(&p.parse().nodes, 0);
}
