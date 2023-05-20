use std::{
    fs::File,
    io::{self, Read},
    iter::Iterator,
    thread,
};

use bytes::{Buf, BytesMut};

const BUF_CAPACITY: usize = 1024 * 8; // 8KB
struct ReadStream<R: Read>(R);
impl Iterator for ReadStream<File> {
    type Item = io::Result<BytesMut>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; BUF_CAPACITY];
        match self.0.read(&mut buf) {
            Ok(0) => None,
            Ok(size) => {
                let buf = BytesMut::from(&buf[..size]);

                Some(Ok(buf))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

pub fn copy(src: &str, dst: &str) -> io::Result<()> {
    let src = File::open(src)?;
    let mut src = ReadStream(src);
    let mut dst = File::create(dst)?;
    let mut buf = None;

    loop {
        if let Some(bytes) = buf.take() {
            let bytes: BytesMut = bytes?;
            let mut chunk = bytes.chunk();
            thread::scope(|s| {
                s.spawn(|| io::copy(&mut chunk, &mut dst));
                s.spawn(|| buf = src.next());
            });
        } else {
            buf = src.next();
            if buf.is_none() {
                break;
            }
        }
    }

    Ok(())
}
