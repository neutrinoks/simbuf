//! A contiguous growable buffer type with heap-allocated contents, written `Buffer`.
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;

/// The default pre-allocation size of `Buffer`.
pub const DEFAULT_INITIALISATION_SIZE: usize = 8192;

/// Macro for easy generation of `Buffer` aligned to `vec` macro.
/// ```
/// # use simbuf::*;
/// let buf = buffer![1, 2, 3];
/// assert_eq!(buf[0], 1);
/// assert_eq!(buf[1], 2);
/// assert_eq!(buf[2], 3);
/// ```
#[macro_export]
macro_rules! buffer {
    () => {
        Buffer::new()
    };
    ($($par:expr),*) => {
        Buffer::from(vec![$($par as u8),*])
    };
    ($init:expr; $nel:expr) => {
        Buffer::from(vec![$init; $nel])
    };
}

/// A contiguous growable buffer, to be used in IO operations and serialisations.
#[derive(Clone, Eq)]
pub struct Buffer {
    /// The internal true buffer.
    data: Vec<u8>,
    /// Pointer to current position to reading.
    rpos: usize,
}

impl Buffer {
    /// Constructs a new, empty `Buffer`.
    ///
    /// The buffer will allocate 8kB of default buffer size.
    /// ```
    /// # use simbuf::Buffer;
    /// let mut buffer = Buffer::new();
    /// ```
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_INITIALISATION_SIZE)
    }

    /// Constructs a new, empty `Buffer` with at least the specified capacity.
    ///
    /// The vector will be able to hold at least capacity elements without reallocating. This method
    /// is allowed to allocate for more elements than capacity. If capacity is 0, the vector will
    /// not allocate.
    ///
    /// It is important to note that although the returned vector has the minimum capacity
    /// specified, the vector will have a zero length. For an explanation of the difference between
    /// length and capacity, see Capacity and reallocation.
    ///
    /// If it is important to know the exact allocated capacity of a `Buffer`, always use the
    /// capacity method after construction.
    /// ```
    /// # use simbuf::Buffer;
    /// let mut buffer = Buffer::with_capacity(128);
    /// ```
    pub fn with_capacity(size: usize) -> Self {
        Self {
            data: Vec::with_capacity(size),
            rpos: 0,
        }
    }

    /// Returns the total number of elements the buffer can hold without reallocating.
    /// ```
    /// # use simbuf::Buffer;
    /// let mut buffer = Buffer::with_capacity(128);
    /// assert_eq!(buffer.capacity(), 128);
    /// ```
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Clears the vector, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity of the vector.
    /// ```
    /// # use simbuf::Buffer;
    /// let mut buf = Buffer::from(vec![1, 2, 3]);
    /// buf.clear();
    /// assert!(buf.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.data.clear();
        self.rpos = 0;
    }

    /// Clones and appends all elements in a slice to the internal buffer.
    ///
    /// Iterates over the slice other, clones each element, and then appends it to this buffer. The
    /// other slice is traversed in-order.
    /// ```
    /// # use simbuf::Buffer;
    /// let mut buf = Buffer::new();
    /// buf.extend_from_slice(&[2, 3, 4]);
    /// assert_eq!(buf.as_slice(), &[2, 3, 4]);
    /// ```
    pub fn extend_from_slice(&mut self, buf: &[u8]) {
        self.data.extend_from_slice(buf);
    }

    /// Clones and appends all elements in the given buffer to this buffer.
    ///
    /// Iterates over the other buffer, clones each element, and then appends it to this buffer.
    /// The other buffer is traversed in-order.
    /// ```
    /// # use simbuf::Buffer;
    /// let mut buf = Buffer::from(vec![1u8]);
    /// let other = Buffer::from(vec![3u8, 5u8, 7u8]);
    /// buf.append(&other);
    /// assert_eq!(buf.as_slice(), &[1, 3, 5, 7]);
    /// ```
    pub fn append(&mut self, other: &Self) {
        self.extend_from_slice(other.as_slice());
    }

    /// Extracts a slice containing the entire vector.
    ///
    /// Equivalent to &s[..].
    /// ```
    /// # use simbuf::Buffer;
    /// use std::io::{self, Write};
    /// let buffer = vec![1, 2, 3, 5, 8];
    /// io::sink().write(buffer.as_slice()).unwrap();
    /// ```
    pub fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }

    /// Extracts a mutable slice of the entire vector.
    ///
    /// Equivalent to &mut s[..].
    /// ```
    /// # use simbuf::*;
    /// use std::io::{self, Read};
    /// let mut buffer = buffer![0, 3];
    /// io::repeat(0b101).read_exact(buffer.as_mut_slice()).unwrap();
    /// ```
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }

    /// Returns an iterator over the internal slice.
    ///
    /// The iterator yields all items from start to end.
    /// ```
    /// # use simbuf::*;
    /// let buf = buffer![1, 2, 4];
    /// let mut iterator = buf.iter();
    ///
    /// assert_eq!(iterator.next(), Some(&1));
    /// assert_eq!(iterator.next(), Some(&2));
    /// assert_eq!(iterator.next(), Some(&4));
    /// assert_eq!(iterator.next(), None);
    /// ```
    pub fn iter(&self) -> alloc::slice::Iter<'_, u8> {
        self.data.iter()
    }

    /// Returns an iterator that allows modifying each value.
    ///
    /// The iterator yields all items from start to end.
    /// ```
    /// # use simbuf::*;
    /// let mut buf = buffer![1, 2, 4];
    /// for elem in buf.iter_mut() {
    ///     *elem += 2;
    /// }
    /// assert_eq!(buf, [3u8, 4u8, 6u8]);
    /// ```
    pub fn iter_mut(&mut self) -> alloc::slice::IterMut<'_, u8> {
        self.data.iter_mut()
    }

    /// Returns true if the vector contains no elements.
    /// ```
    /// # use simbuf::*;
    /// let mut buf = Buffer::new();
    /// assert!(buf.is_empty());
    ///
    /// buf.push(1);
    /// assert!(!buf.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Appends an element to the back of a collection.
    /// ```
    /// # use simbuf::*;
    /// let mut buf = buffer![1, 2];
    /// buf.push(3);
    /// assert_eq!(buf, [1u8, 2u8, 3u8]);
    /// ```
    pub fn push(&mut self, b: u8) {
        self.data.push(b);
    }

    // TODO: much more methods to write bytes
    pub fn push_u16(&mut self, val: u16) {
        todo!();
    }

    pub fn push_u16_be(&mut self, val: u16) {
        todo!();
    }

    pub fn push_u16_le(&mut self, val: u16) {
        todo!();
    }

    pub fn push_i16(&mut self, val: u16) {
        todo!();
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<u8>> for Buffer {
    fn from(val: Vec<u8>) -> Self {
        Self { data: val, rpos: 0 }
    }
}

impl From<&[u8]> for Buffer {
    fn from(val: &[u8]) -> Self {
        let mut buf = Self::new();
        buf.extend_from_slice(val);
        buf
    }
}

impl PartialEq for Buffer {
    #[inline]
    fn eq(&self, other: &Buffer) -> bool {
        self.data == other.data
    }
}

impl PartialEq<[u8]> for Buffer {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        self.data.as_slice() == other
    }
}

impl<const N: usize> PartialEq<[u8; N]> for Buffer {
    #[inline]
    fn eq(&self, other: &[u8; N]) -> bool {
        self.data[..] == other[..]
    }
}

impl core::convert::AsMut<[u8]> for Buffer {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }
}

impl core::convert::AsRef<[u8]> for Buffer {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.data.as_slice()
    }
}

impl core::fmt::Debug for Buffer {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "Buffer {:?}", self.data)
    }
}

impl core::ops::Index<usize> for Buffer {
    type Output = u8;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl core::iter::IntoIterator for Buffer {
    type Item = u8;
    type IntoIter = alloc::vec::IntoIter<u8>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[cfg(feature = "std")]
impl std::io::Write for Buffer {
    fn write(&mut self, bytes: &[u8]) -> Result<usize, std::io::Error> {
        self.data.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

#[cfg(feature = "std")]
impl std::io::Read for Buffer {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let len = core::cmp::min(buf.len(), self.data.len() - self.rpos);
        buf.copy_from_slice(&self.data[self.rpos..self.rpos + len]);
        self.rpos += len;
        Ok(len)
    }
}

#[cfg(feature = "parity-scale-codec")]
impl codec::Input for Buffer {
    fn remaining_len(&mut self) -> Result<Option<usize>, codec::Error> {
        Ok(Some(self.data.len() - self.rpos))
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<(), codec::Error> {
        let len = core::cmp::min(buf.len(), self.data.len() - self.rpos);
        buf.copy_from_slice(&self.data[self.rpos..self.rpos + len]);
        Ok(())
    }
}

#[cfg(all(feature = "parity-scale-codec", not(feature = "std")))]
impl codec::Output for Buffer {
    fn write(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn macro_test() {
        let buf = buffer!(3, 5, 7);
        assert_eq!(buf, Buffer::from(vec![3u8, 5u8, 7u8]));
    }

    #[test]
    fn smoke_test_new() {
        assert_eq!(
            Buffer::new(),
            Buffer {
                data: Vec::with_capacity(DEFAULT_INITIALISATION_SIZE),
                rpos: 0,
            }
        );
    }

    #[test]
    fn smoke_test_with_capacity() {
        assert_eq!(
            Buffer::with_capacity(128),
            Buffer {
                data: Vec::with_capacity(128),
                rpos: 0,
            }
        );
    }

    #[test]
    fn smoke_test_iter() {
        let buf = Buffer::from(vec![0u8, 2u8, 4u8]);
        let mut iter = buf.iter();
        assert_eq!(iter.next(), Some(&0u8));
        assert_eq!(iter.next(), Some(&2u8));
        assert_eq!(iter.next(), Some(&4u8));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn smoke_test_iter_mut() {
        let mut buf = Buffer::from(vec![3u8, 2u8, 3u8]);
        buf.iter_mut().for_each(|b| {
            if *b != 3 {
                *b = 4;
            }
        });
        assert_eq!(&[3u8, 4u8, 3u8], buf.as_slice());
    }

    #[test]
    fn smoke_test_into_iter() {
        let buf = Buffer::from(vec![2u8, 4u8, 8u8]);
        let mut iter = buf.into_iter();
        assert_eq!(Some(2u8), iter.next());
        assert_eq!(Some(4u8), iter.next());
        assert_eq!(Some(8u8), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn smoke_test_default() {
        assert_eq!(
            Buffer::new(),
            Buffer {
                data: Vec::with_capacity(DEFAULT_INITIALISATION_SIZE),
                rpos: 0,
            }
        );
    }

    #[test]
    fn smoke_test_from_vec() {
        let vec = vec![1u8, 2u8, 4u8];
        let buf = Buffer::from(vec);
        assert_eq!(&[1u8, 2u8, 4u8], buf.as_slice());
    }

    #[test]
    fn smoke_test_from_slice() {
        let vec = vec![1u8, 2u8, 4u8];
        let buf = Buffer::from(&vec[..]);
        assert_eq!(&[1u8, 2u8, 4u8], buf.as_slice());
    }

    #[test]
    fn smoke_test_debug() {
        let mut buf = Buffer::new();
        buf.extend_from_slice(&[1, 2, 3]);
        let fmt = format!("{buf:?}");
        assert_eq!(fmt, "Buffer [1, 2, 3]".to_string());
    }

    #[test]
    fn smoke_test_as_ref() {
        let mut buf = Buffer::new();
        buf.extend_from_slice(&[1, 2, 3]);
        assert_eq!(&[1u8, 2u8, 3u8], AsRef::<[u8]>::as_ref(&buf));
    }

    #[test]
    fn smoke_test_as_mut() {
        let mut buf = Buffer::new();
        buf.extend_from_slice(&[1, 2, 3]);
        let buf_ref = buf.as_mut();
        buf_ref[1] = 4;
        assert_eq!(&[1u8, 4u8, 3u8], AsRef::<[u8]>::as_ref(&buf));
    }
}
