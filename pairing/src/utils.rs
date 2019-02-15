use IoError;

use rstd::prelude::*;
use rstd::cmp;
use rstd::ptr::copy_nonoverlapping;

macro_rules! write_num_bytes {
    ($ty:ty, $size:expr, $n:expr, $dst:expr, $which:ident) => ({
        assert!($size <= $dst.len());
        unsafe {
            // N.B. https://github.com/rust-lang/rust/issues/22776
            let bytes = *(&$n.$which() as *const _ as *const [u8; $size]);
            copy_nonoverlapping((&bytes).as_ptr(), $dst.as_mut_ptr(), $size);
        }
    });
}

macro_rules! read_num_bytes {
    ($ty:ty, $size:expr, $src:expr, $which:ident) => ({
        assert!($size == ::core::mem::size_of::<$ty>());
        assert!($size <= $src.len());
        let mut data: $ty = 0;
        unsafe {
            copy_nonoverlapping(
                $src.as_ptr(),
                &mut data as *mut $ty as *mut u8,
                $size);
        }
        data.$which()
    });
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, IoError>;
    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), IoError> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => return Err(IoError::WriteZero),
                Ok(n) => buf = &buf[n..],
                // Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
    fn write_u32(&mut self, n: u32) -> Result<(), IoError>;
    fn write_u64(&mut self, n: u64) -> Result<(), IoError>;
}

impl Write for Vec<u8> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, IoError> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> Result<(), IoError> {
        self.extend_from_slice(buf);
        Ok(())
    }    

    #[inline]
    fn write_u32(&mut self, n: u32) -> Result<(), IoError> {
        let mut buf = [0; 4];
        write_u32_be(&mut buf, n);
        self.write_all(&buf)
    }

    #[inline]
    fn write_u64(&mut self, n: u64) -> Result<(), IoError> {
        let mut buf = [0; 8];
        write_u64_be(&mut buf, n);
        self.write_all(&buf)
    }
}

#[inline]
fn write_u32_be(buf: &mut [u8], n: u32) {
    write_num_bytes!(u32, 4, n, buf, to_be);
}

#[inline]
fn write_u64_be(buf: &mut [u8], n: u64) {
    write_num_bytes!(u64, 8, n, buf, to_be);
}

#[inline]
fn read_u32_be(buf: &[u8]) -> u32 {
    read_num_bytes!(u32, 4, buf, to_be)
}

#[inline]
fn read_u64_be(buf: &[u8]) -> u64 {
    read_num_bytes!(u64, 8, buf, to_be)
}

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError>;    
    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<(), IoError> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => break,
                Ok(n) => { let tmp = buf; buf = &mut tmp[n..]; }
                // Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        if !buf.is_empty() {
            Err(IoError::Error)
        } else {
            Ok(())
        }
    }
    fn read_u32(&mut self) -> Result<u32, IoError>;
    fn read_u64(&mut self) -> Result<u64, IoError>;
}

impl<'a> Read for &'a [u8] {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
        let amt = cmp::min(buf.len(), self.len());
        let (a, b) = self.split_at(amt);

        // First check if the amount of bytes we want to read is small:
        // `copy_from_slice` will generally expand to a call to `memcpy`, and
        // for a single byte the overhead is significant.
        if amt == 1 {
            buf[0] = a[0];
        } else {
            buf[..amt].copy_from_slice(a);
        }

        *self = b;
        Ok(amt)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), IoError> {
        if buf.len() > self.len() {
            // return Err(Error::new(ErrorKind::UnexpectedEof,
                                //   "failed to fill whole buffer"));
            return Err(IoError::Error);
        }
        let (a, b) = self.split_at(buf.len());

        // First check if the amount of bytes we want to read is small:
        // `copy_from_slice` will generally expand to a call to `memcpy`, and
        // for a single byte the overhead is significant.
        if buf.len() == 1 {
            buf[0] = a[0];
        } else {
            buf.copy_from_slice(a);
        }

        *self = b;
        Ok(())
    }

    #[inline]
    fn read_u64(&mut self) -> Result<u64, IoError> {
        let mut buf = [0; 8];
        try!(self.read_exact(&mut buf));
        Ok(read_u64_be(&buf))
    }

    #[inline]
    fn read_u32(&mut self) -> Result<u32, IoError> {
        let mut buf = [0; 4];
        try!(self.read_exact(&mut buf));
        Ok(read_u32_be(&buf))
    }
}
