//! Provides a trait for `std::io::Result` that adds a method making
//! it easy to tell the difference between a file not found and
//! another error, since a common practice is to handle a file if it
//! exists.
//!
//! # Examples
//! ````
//! use io_result_optional::IoResultOptional;
//! use std::fs::File;
//! # use std::io;
//!
//! # fn readconfig(data: File) -> u8 {
//! #     17
//! # }
//! # fn main() -> io::Result<()> {
//! let config = File::open(".app.rc")
//!     .optional()?
//!     .map(readconfig)
//!     .unwrap_or_default();
//! # Ok(())
//! # }
//! ````
//!
//! ````
//! use io_result_optional::IoResultOptional;
//! use std::fs::File;
//! # use std::io;
//!
//! # fn main() -> io::Result<()> {
//! if let Some(input) = File::open("data").optional()? {
//!     // The data exists, so handle it ...
//!     // If it doesn't exist, it is just ignored
//!     // If there is another error, this function returns it.
//! }
//! # Ok(())
//! # }
//! ````
use std::io;

/// A trait for [`io::Result`] that adds a method making it easy to
/// tell the difference between a file not found and another error,
/// since a common practice is to handle a file if it exists.
pub trait IoResultOptional<T> {
    /// Consider the given file access optional.
    /// if the result is an error with [`io::ErrorKind`] `NotFound`, convert it to
    /// `Ok(None)`.
    /// If it is any other error, return it as-is,
    /// and if it is `Ok(value)` convert it to `Ok(Some(value))`.
    ///
    /// # Examples
    /// ````
    /// use std::fs::File;
    /// # use std::io;
    /// use io_result_optional::IoResultOptional;
    ///
    /// # fn parseconfig(data: File) -> u8 {
    /// #     17
    /// # }
    /// # fn main() -> io::Result<()> {
    /// let config = File::open(".app.rc")
    ///     .optional()?
    ///     .map(parseconfig)
    ///     .unwrap_or_default();
    /// # Ok(())
    /// # }
    /// ````
    fn optional(self) -> io::Result<Option<T>>;
}

impl<T> IoResultOptional<T> for io::Result<T> {
    fn optional(self) -> io::Result<Option<T>> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::IoResultOptional;
    use std::fs::File;
    use std::io;
    use std::path::Path;

    #[test]
    fn existing_some() {
        assert!(
            File::open(&Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
                .optional()
                .unwrap()
                .is_some()
        )
    }

    #[test]
    fn non_existing_none() {
        assert!(
            File::open(&Path::new(env!("CARGO_MANIFEST_DIR")).join("nosuch.file"))
                .optional()
                .unwrap()
                .is_none()
        )
    }

    #[test]
    fn other_is_error() {
        let result: io::Result<()> = Err(io::Error::new(io::ErrorKind::TimedOut, "too slow"));
        assert_eq!(
            format!("{:?}", result.optional()),
            "Err(Custom { kind: TimedOut, error: StringError(\"too slow\") })",
        )
    }
}
