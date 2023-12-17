use std::str::from_utf8;
use crate::lazy_reader::LazyReader;

#[derive(Debug)]
pub enum Token {
    String(String),
    LArrow,
    RArrow,
    Dot,
    FSlash,
    Eq,
    Quote,
}

pub struct Lexer {
    buf: Vec<u8>,
    cursor: usize,
    reader: LazyReader,
}

impl Lexer {
    pub fn new(reader: LazyReader) -> Self {
        Self { reader, buf: vec![], cursor: 0 }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.peak() {
            match ch {
                b' ' | b'\n' | b'\t' => {
                    self.consume();
                }
                b'<' => {
                    tokens.push(Token::LArrow);
                    self.consume();
                }
                b'>' => {
                    tokens.push(Token::RArrow);
                    self.consume();
                }
                b'/' => {
                    tokens.push(Token::FSlash);
                    self.consume();
                }
                b'=' => {
                    tokens.push(Token::Eq);
                    self.consume();
                }
                b'"' => {
                    tokens.push(Token::Quote);
                    self.consume();
                }
                b'.' => {
                    tokens.push(Token::Dot);
                    self.consume();
                }
                b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' => {
                    tokens.push(Token::String(self.consume_str()));
                }
                _ => {
                    println!("Unknown: {:?}", from_utf8(&[ch]));
                    self.consume();
                }
            }
        }

        tokens
    }

    fn consume_str(&mut self) -> String {
        let mut tmp_str = Vec::new();

        while let Some(ch) = self.peak() {
            if ch.is_ascii_alphanumeric() {
                tmp_str.push(ch);
                self.consume();
            } else { break; }
        }
        String::from_utf8(tmp_str).expect("must be a valid string")
    }

    fn peak(&mut self) -> Option<u8> {
        if self.buf.len() == 0 {
            self.buf = self.reader.next_chunk()?;
        }

        if self.cursor >= self.buf.len() {
            self.buf = self.reader.next_chunk()?;
            self.cursor = 0;
            return match self.buf.get(0) {
                None => None,
                Some(ch) => { Some(*ch) }
            };
        }

        match self.buf.get(self.cursor) {
            None => None,
            Some(ch) => { Some(*ch) }
        }
    }

    fn consume(&mut self) -> Option<u8> {
        if let Some(ch) = self.peak() {
            self.cursor += 1;
            Some(ch);
        }
        None
    }
}
