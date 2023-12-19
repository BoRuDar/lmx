mod lazy_reader;
mod lexer;
mod parser;

use std::fs::File;
use lazy_reader::LazyReader;
use lexer::{Lexer};
use parser::Parser;

fn main() {
    let Ok(file) = File::open("./files/test3.xml") else {
        panic!("fail to open the file");
    };
    let lr = LazyReader::new(Box::new(file), 32);
    let Some(mut p) = Parser::new(Lexer::new(lr).parse().as_slice()) else { panic!("todo") };
    dbg!(p.parse());
}

