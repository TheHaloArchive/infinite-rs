//! Extensions to [`BufRead`] for reading fixed-length strings and enumerable types.
//!
//! This module provides two main extensions to the standard [`BufRead`]:
//!
//! * [`read_fixed_string`](`BufReaderExt::read_fixed_string`): Reads a fixed number of bytes and converts them to a UTF-8 string.
//!   Special handling is included for sequences of `0xFF` bytes which are treated as empty strings.
//!
//! * [`read_enumerable`](`BufReaderExt::read_enumerable`): Generic method for reading a sequence of items that implement the
//!   [`Enumerable`] trait. Reads the specified type `count` times and collects the results into a [`Vec`].
//!
//! These extensions are implemented as traits and require the reader to implement both
//! [`Read`] and [`Seek`] traits.
//!

use std::io::{BufRead, BufReader, Read, Seek};

use crate::Result;

/// Trait for types that can be read sequentially from a buffered reader.
///
/// Types implementing this trait can be read using the [`read_enumerable`](`BufReaderExt::read_enumerable`) method
/// from [`BufReaderExt`].
pub trait Enumerable {
    /// Reads data from the given reader and updates the implementing type.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to any type that implements `BufReaderExt`
    ///
    /// # Errors
    /// - If the reader fails to read the exact number of bytes [`ReadError`](`crate::Error::ReadError`)
    fn read<R: BufReaderExt>(&mut self, reader: &mut R) -> Result<()>;
}

/// Extension trait for [`BufRead`] to add custom reading methods.
pub trait BufReaderExt: BufRead + Seek {
    /// Reads a fixed-length UTF-8 encoded string from the reader.
    ///
    /// This function reads exactly `length` bytes and converts them to a String.
    /// If the bytes read are all 0xFF, an empty string is returned.
    ///
    /// # Arguments
    ///
    /// * `length` - The exact number of bytes to read
    ///
    /// # Errors
    /// - If the reader fails to read the exact number of bytes [`ReadError`](`crate::Error::ReadError`)
    /// - If the bytes read are not valid UTF-8 [`Utf8ReadingError`](`crate::Error::Utf8ReadingError`)
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    /// use std::io::BufReader;
    /// use infinite_rs::common::extensions::BufReaderExt;
    ///
    /// let data = b"I love cats!";
    /// let mut reader = BufReader::new(Cursor::new(data));
    /// let string = reader.read_fixed_string(data.len()).unwrap();
    /// assert_eq!(string, "I love cats!");
    /// ```
    fn read_fixed_string(&mut self, length: usize) -> Result<String> {
        let mut buffer = vec![0; length];
        self.read_exact(&mut buffer)?;

        Ok(if buffer == [0xFF; 4] {
            String::new() // Return empty string if all bytes are 0xFF
        } else {
            String::from_utf8(buffer)?
        })
    }

    /// Reads a null-terminated string from the reader.
    ///
    /// This function reads bytes in a reader until it hits `0x00` and converts them to a String.
    /// The null terminator is removed from the final output.
    ///
    /// # Errors
    /// - If the reader fails to read the exact number of bytes [`ReadError`](`crate::Error::ReadError`)
    /// - If the bytes read are not valid UTF-8 [`Utf8ReadingError`](`crate::Error::Utf8ReadingError`)
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    /// use std::io::BufReader;
    /// use infinite_rs::common::extensions::BufReaderExt;
    ///
    /// let data = [0x49, 0x20, 0x6c, 0x6f, 0x76, 0x65, 0x20, 0x63, 0x61, 0x74, 0x73, 0x21, 0x00];
    /// let mut reader = BufReader::new(Cursor::new(data));
    /// let string = reader.read_null_terminated_string().unwrap();
    /// assert_eq!(string, "I love cats!");
    /// ```
    fn read_null_terminated_string(&mut self) -> Result<String> {
        let mut buffer = Vec::with_capacity(150); // Pre-allocate around 150 bytes (typical
        // filename size)
        self.read_until(0x00, &mut buffer)?;
        buffer.pop(); // remove null terminator

        let string = String::from_utf8(buffer)?;

        Ok(string)
    }

    /// Reads multiple instances of an enumerable type into a vector.
    ///
    /// Creates a vector of type T by reading the type `count` times from the buffer.
    /// Type T must implement both [`Default`] and [`Enumerable`]    traits.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to read, must implement `Default + Enumerable`
    ///
    /// # Arguments
    ///
    /// * `count` - Number of instances to read
    ///
    /// # Errors
    /// - If the reader fails to read the exact number of bytes [`ReadError`](`crate::Error::ReadError`)
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{Cursor, BufReader,};
    /// use infinite_rs::common::extensions::{BufReaderExt, Enumerable};
    /// use infinite_rs::common::errors::Error;
    /// use byteorder::{ReadBytesExt, LE};
    ///
    /// #[derive(Default)]
    /// struct TestType {
    ///     value: u32,
    /// }
    ///
    /// impl Enumerable for TestType {
    ///     fn read<R: BufReaderExt>(&mut self, reader: &mut R) -> Result<(), Error> {
    ///         self.value = reader.read_u32::<LE>()?;
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let data = b"\x01\x00\x00\x00\x02\x00\x00\x00\x03\x00\x00\x00";
    /// let mut reader = BufReader::new(Cursor::new(data));
    /// let enumerables = reader.read_enumerable::<TestType>(3).unwrap();
    /// assert_eq!(enumerables.len(), 3);
    /// assert_eq!(enumerables[0].value, 1);
    /// assert_eq!(enumerables[1].value, 2);
    /// assert_eq!(enumerables[2].value, 3);
    /// ```
    fn read_enumerable<T: Default + Enumerable>(&mut self, count: u64) -> Result<Vec<T>>
    where
        Self: Sized,
        Vec<T>: FromIterator<T>,
    {
        let mut enumerables = vec![];
        enumerables.reserve_exact(usize::try_from(count)? + 1);
        for _ in 0..count {
            let mut enumerable = T::default();
            enumerable.read(self)?;
            enumerables.push(enumerable);
        }
        Ok(enumerables)
    }
}

impl<R: Read + Seek> BufReaderExt for BufReader<R> {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    /// Verifies that reading 0xFFFFFFFF returns an empty string, which is used
    /// to handle empty `tag_group` entries in module files.
    fn test_read_fixed_string_empty() {
        let data = [255, 255, 255, 255];
        let mut reader = BufReader::new(Cursor::new(&data));
        let string = reader.read_fixed_string(data.len()).unwrap();
        assert_eq!(string, "");
    }
}
