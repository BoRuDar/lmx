use std::io::Read;

pub struct LazyReader {
    source: Box<dyn Read>,
    buf: Box<Vec<u8>>,
}

impl LazyReader {
    pub fn new(source: Box<impl Read + 'static>, size: usize) -> Self <> {
        Self { source, buf: Box::new(vec![0; size]) }
    }

    pub fn next_chunk(&mut self) -> Option<Vec<u8>> {
        match self.read() {
            Some(0) | None => None,
            Some(n) => {
                Some(self.buf.as_slice()[..n].to_vec())
            }
        }
    }

    /// Read up to 'size' bytes into internal buffer.
    fn read(&mut self) -> Option<usize> {
        match self.source.read(self.buf.as_mut_slice()) {
            Ok(n) => { Some(n) }
            Err(e) => panic!("read failed: {}", e)
        }
    }
}
