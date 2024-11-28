//! This crates implements a very simple byte buffer type. It is basically a `Vec` added by two
//! counters for read and write marker storing.
//!
//! # `Buffer`
//!
//! TODO more descriptive examples
//!
//! ```
//! use simbuf::Buffer;
//! use std::io::Write;
//!
//! let mut buffer = Buffer::new();
//! buffer.write(b"Hello, world!").unwrap();
//!
//! assert_eq!(AsRef::<[u8]>::as_ref(&buffer), b"Hello, world!");
//! ```

/// The default initial size of the buffer in bytes.
const INITIAL_SIZE: usize = 8192;
/// The bin size of allocations in case the buffer is too small.
const ALLOC_SIZE: usize = 2048;

/// The buffer with internal data storage, read marker and write marker.
///
/// TODO
///
/// # Examples
/// ```
/// // TODO
/// ```
///
/// # Indexin
/// ```
/// // TODO
/// ```
///
/// # Slicing
/// ```
/// // TODO
/// ```
///
/// # Capacity and reallocation
///
/// Currently, the initial allocated size is 8192 bytes, similar to writing `vec![0u8; 8192]`. When
/// writing to the buffer it automatically allocates 2048 new bytes in case of reaching the current
/// allocation size. The new allocated bytes are filled with zeros.
///
/// This behavior is to enable writing data by using trait `std::io::Write`.
///
/// # Read- and write-marker
///
/// TODO
#[derive(Clone, Eq)]
pub struct Buffer {
    /// True data buffer.
    data: Vec<u8>,
    /// Read marker when reading.
    read: usize,
    /// Write marker when writing.
    write: usize,
}

impl Buffer {
    /// New type pattern. Initializes a new buffer with 8192 bytes.
    ///
    /// # Example
    /// ```
    /// # use simbuf::Buffer;
    /// let buffer = Buffer::new();
    /// ```
    pub fn new() -> Self {
        Self {
            data: vec![0; INITIAL_SIZE],
            read: 0,
            write: 0,
        }
    }

    /// Pushes a single byte at the current write position of the buffer.
    ///
    /// # Example
    pub fn push(&mut self, b: u8) {
        self.check_allocation(1);
        self.data[self.write] = b;
        self.write += 1;
    }

    /// Appends the given slice at the current write position of the buffer.
    pub fn append(&mut self, data: &[u8]) {
        self.check_allocation(data.len());
        self.data[self.write..self.write + data.len()].copy_from_slice(data);
        self.write += data.len();
    }

    /// Tries to move the read marker position forward by `seek` positions until it reaches the
    /// write marker position.
    pub fn seek(&mut self, seek: usize) {
        self.read += core::cmp::min(self.read + seek, self.write);
    }

    /// TODO
    pub fn clear(&mut self) {
        self.read = 0;
        self.write = 0;
    }

    /// Returns the number of elements in the buffer, also referred to as its write position.
    ///
    /// # Example
    /// ```
    /// # use simbuf::Buffer;
    /// let mut buffer = Buffer::new();
    /// buffer.append(&[1, 2, 3]);
    /// assert_eq!(buffer.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.write
    }

    /// TODO
    pub fn is_empty(&self) -> bool {
        self.write == 0
    }

    /// TODO
    pub fn first(&self) -> u8 {
        self.data[0]
    }

    /// TODO
    pub fn first_n(&self, n: usize) -> &[u8] {
        &self.data[..n]
    }

    /// TODO
    pub fn last(&self) -> Option<u8> {
        if self.is_empty() {
            None
        } else {
            Some(self.data[self.write - 1])
        }
    }

    /// TODO
    pub fn last_n(&self, n: usize) -> Option<&[u8]> {
        if n > self.write {
            None
        } else {
            Some(&self.data[self.write - n..self.write])
        }
    }

    #[inline]
    fn from_slice_int(src: &[u8]) -> Self {
        let write = src.len();
        let read = 0;
        let len = core::cmp::max(write, INITIAL_SIZE);
        let mut data = vec![0u8; len];
        data[..write].copy_from_slice(src);
        Self { data, read, write }
    }

    #[inline]
    fn check_allocation(&mut self, dlen: usize) {
        if self.data.len() < self.write + dlen {
            let nalloc = if dlen < ALLOC_SIZE {
                ALLOC_SIZE
            } else {
                ((dlen / ALLOC_SIZE) + 1) * ALLOC_SIZE
            };
            self.data.extend_from_slice(&vec![0; nalloc]);
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn read_from_async<S>(&mut self, source: &mut S) -> std::io::Result<usize>
    where
        S: tokio::io::AsyncReadExt + core::marker::Unpin,
    {
        let n_bytes = source.read(&mut self.data[self.write..]).await?;
        self.write += n_bytes;
        Ok(n_bytes)
    }

    #[cfg(feature = "tokio")]
    pub async fn write_to_async<S>(&mut self, sink: &mut S) -> std::io::Result<usize>
    where
        S: tokio::io::AsyncWriteExt + core::marker::Unpin,
    {
        sink.write_all(&mut self.data[self.read..self.write])
            .await?;
        let n_bytes = self.write - self.read;
        self.read = self.write;
        Ok(n_bytes)
    }
}

impl core::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{:?}", &self.data[..self.write])
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: core::slice::SliceIndex<[u8]>> core::ops::Index<I> for Buffer {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        core::ops::Index::index(&**self, index)
    }
}

impl core::ops::Deref for Buffer {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.data.as_slice()
    }
}

impl core::convert::AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        &self.data[self.read..self.write]
    }
}

impl PartialEq for Buffer {
    fn eq(&self, other: &Self) -> bool {
        self.data[..self.write] == other.data[..other.write]
    }
}

impl PartialEq<[u8]> for Buffer {
    fn eq(&self, other: &[u8]) -> bool {
        self.data[self.read..self.write] == *other
    }
}

impl<const N: usize> PartialEq<[u8; N]> for Buffer {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.data[self.read..self.write] == *other
    }
}

impl PartialEq<Vec<u8>> for Buffer {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.data[self.read..self.write] == *other
    }
}

impl From<&[u8]> for Buffer {
    fn from(value: &[u8]) -> Self {
        Self::from_slice_int(value)
    }
}

impl<const N: usize> From<[u8; N]> for Buffer {
    fn from(value: [u8; N]) -> Self {
        Self::from_slice_int(value.as_slice())
    }
}

impl From<Vec<u8>> for Buffer {
    fn from(value: Vec<u8>) -> Self {
        Self::from_slice_int(&value)
    }
}

#[cfg(feature = "std")]
impl std::io::Write for Buffer {
    fn write(&mut self, data: &[u8]) -> std::result::Result<usize, std::io::Error> {
        self.append(data);
        Ok(data.len())
    }

    fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
        Ok(())
    }
}

#[cfg(feature = "std")]
impl std::io::Read for Buffer {
    fn read(&mut self, buf: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
        let alen = self.write - self.read;
        let rlen = std::cmp::min(alen, buf.len());
        buf[..rlen].copy_from_slice(&self.data[self.read..self.read + alen]);
        Ok(rlen)
    }
}

#[cfg(test)]
mod tests {
    use super::Buffer;
    use std::io::{Read, Write};

    const REFDATA: [u8; 5] = [1, 2, 4, 8, 16];

    #[test]
    fn cmp_in_different_states() {
        let mut buffer = Buffer::new();
        buffer.append(&REFDATA[..]);
        assert_eq!(&buffer, &REFDATA);
        buffer.clear();
        buffer.append(&REFDATA[..3]);
        assert_eq!(&buffer, &REFDATA[..3]);
    }

    #[test]
    fn from_and_cmp_with_slice() {
        let buffer = Buffer::from(REFDATA.as_slice());
        assert_eq!(&buffer, REFDATA.as_slice());
        let buffer = Buffer::from(REFDATA);
        assert_eq!(&buffer, &REFDATA.to_vec());
        let buffer = Buffer::from(REFDATA);
        assert_eq!(&buffer, &REFDATA);
    }

    #[test]
    fn as_ref_and_seek() {
        let mut buffer = Buffer::from("Hello, world!".as_bytes());
        assert_eq!(buffer.as_ref(), "Hello, world!".as_bytes());
        buffer.seek(7);
        assert_eq!(buffer.as_ref(), "world!".as_bytes());
    }

    #[cfg(feature = "std")]
    #[test]
    fn std_io_write() {
        let mut buffer = Buffer::new();
        buffer.write(REFDATA.as_slice()).unwrap();
        assert_eq!(&buffer, REFDATA.as_slice());
    }

    #[cfg(feature = "std")]
    #[test]
    fn std_io_read() {
        let mut vbuf = vec![0u8; 256];
        let mut buffer = Buffer::from(REFDATA);
        buffer.read(&mut vbuf).unwrap();
        assert_eq!(&vbuf[..5], REFDATA.as_slice());
    }
}
