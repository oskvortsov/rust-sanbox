// https://exercism.org/tracks/rust/exercises/paasio

// topics #[Write, Read - traits, wrappers]

use std::io::{Read, Result, Write};
// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R> {
    data: R,
    bytes: usize,
    reads: usize
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: R) -> ReadStats<R> {
        Self {data: _wrapped, bytes: 0, reads: 0}
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        // todo!("Collect statistics about this call reading {buf:?}")
        let n = self.data.read(buf)?;

        self.reads += 1;
        self.bytes += n;

        Ok(n)
    }
}

pub struct WriteStats<W> {
    data: W,
    bytes: usize,
    writes: usize
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: W) -> WriteStats<W> {
        Self {
            data: _wrapped,
            bytes: 0,
            writes: 0
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = self.data.write(buf)?;

        self.bytes += n;
        self.writes += 1;

        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}
