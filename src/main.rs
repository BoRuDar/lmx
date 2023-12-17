mod lazy_reader;
mod lexer;

use lazy_reader::LazyReader;
use lexer::Lexer;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::open("./files/test3.xml")?;
    let lr = LazyReader::new(Box::new(file), 32);
    let mut lex = Lexer::new(lr);

    dbg!(lex.parse());

    Ok(())
}

