use std::str::from_utf8;
use crate::lazy_reader::LazyReader;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    String(String),
    OpenNode(String),
    Quote(String),
    CloseNodeNamed(String),
    CloseNode,
    EndOfOpenNode,
    Eq,
    EOF,
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
                    self.consume();
                    if let Some(next) = self.peak() {
                        if next == b'/' {
                            self.consume();
                            tokens.push(Token::CloseNodeNamed(self.consume_str()));
                            self.consume();
                        } else {
                            tokens.push(Token::OpenNode(self.consume_str()));
                        }
                    }
                }
                b'/' => {
                    self.consume();
                    if let Some(next) = self.peak() {
                        if next == b'>' {
                            tokens.push(Token::CloseNode);
                            self.consume();
                        } else {
                            panic!("invalid syntax")
                        }
                    }
                }
                b'>' => {
                    tokens.push(Token::EndOfOpenNode);
                    self.consume();
                }
                b'=' => {
                    tokens.push(Token::Eq);
                    self.consume();
                }
                b'"' => {
                    tokens.push(Token::Quote(self.consume_quote()));
                }
                c if c.is_ascii_alphanumeric() => {
                    tokens.push(Token::String(self.consume_str()));
                }
                _ => {
                    println!("Unknown: {:?}", from_utf8(&[ch]));
                    self.consume();
                }
            }
        }

        tokens.push(Token::EOF);
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

    fn consume_quote(&mut self) -> String {
        self.consume();
        let mut tmp_str = Vec::new();

        while let Some(ch) = self.peak() {
            if ch == b'"' {
                self.consume();
                break;
            }
            tmp_str.push(ch);
            self.consume();
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
