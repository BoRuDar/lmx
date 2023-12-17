mod lazy_reader;
mod lexer;

use lazy_reader::LazyReader;
use lexer::Lexer;
use std::fs::File;

fn main() {
    let Ok(file) = File::open("./files/test3.xml") else {
        panic!("fail to open the file");
    };
    let lr = LazyReader::new(Box::new(file), 32);
    let mut lex = Lexer::new(lr);

    dbg!(lex.parse());
}

