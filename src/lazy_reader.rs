use std::io::Read;

pub struct LazyReader {
    source: Box<dyn Read>,
    buf: Box<Vec<u8>>,
    size: usize,
}

impl LazyReader {
    pub fn new(source: Box<impl Read + 'static>, size: usize) -> Self <> {
        Self {
            source,
            size,
            buf: Box::new((0..size).into_iter().map(|_| 0).collect()),
        }
    }

    pub fn next_chunk(&mut self) -> Option<Vec<u8>> {
        match self.read() {
            Some(0) | None => None,
            Some(n) => {
                Some(self.buf.as_slice()[..n].to_vec())
            }
        }
    }

    fn read(&mut self) -> Option<usize> {
        match self.source.read(self.buf.as_mut_slice()) {
            Ok(n) => { Some(n) }
            Err(e) => panic!("Read failed: {}", e)
        }
    }
}
