use pairing::{     
    IoError,    
};

use rstd::prelude::*;
use rstd::{cmp, mem};

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
}

// impl<'a> Write for &'a mut [u8] {
//     #[inline]
//     fn write(&mut self, data: &[u8]) -> Result<usize, IoError> {
//         let amt = cmp::min(data.len(), self.len());
//         let (a, b) = mem::replace(self, &mut []).split_at_mut(amt);
//         a.copy_from_slice(&data[..amt]);
//         *self = b;
//         Ok(amt)
//     }

//     #[inline]
//     fn write_all(&mut self, data: &[u8]) -> Result<(), IoError> {
//         if self.write(data)? == data.len() {
//             Ok(())
//         } else {
//             // Err(Error::new(ErrorKind::WriteZero, "failed to write whole buffer"))
//             Err(IoError::WriteZero)
//         }
//     }    
// }

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
}

impl<'a> Read for &'a [u8] {
    // #[inline]
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

    // #[inline]
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
}



// impl<'a> Read for &'a mut [u8] {
//     // #[inline]
//     fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
//         let amt = cmp::min(buf.len(), self.len());
//         let (a, b) = self.split_at(amt);

//         // First check if the amount of bytes we want to read is small:
//         // `copy_from_slice` will generally expand to a call to `memcpy`, and
//         // for a single byte the overhead is significant.
//         if amt == 1 {
//             buf[0] = a[0];
//         } else {
//             buf[..amt].copy_from_slice(a);
//         }

//         *self = b.as_mut();
//         Ok(amt)
//     }

//     // #[inline]
//     fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), IoError> {
//         if buf.len() > self.len() {
//             // return Err(Error::new(ErrorKind::UnexpectedEof,
//                                 //   "failed to fill whole buffer"));
//             return Err(IoError::Error);
//         }
//         let (a, b) = self.split_at(buf.len());

//         // First check if the amount of bytes we want to read is small:
//         // `copy_from_slice` will generally expand to a call to `memcpy`, and
//         // for a single byte the overhead is significant.
//         if buf.len() == 1 {
//             buf[0] = a[0];
//         } else {
//             buf.copy_from_slice(a);
//         }

//         *self = b;
//         Ok(())
//     }
// }
