use libarchive_sys::*;
use std::{
    ffi::{CString, NulError},
    os::raw::c_char,
    path::Path,
    ptr::NonNull,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("LibArchive error: {0}")]
    LibArchive(u32),
    #[error("FFI error: {0}")]
    NulError(#[from] NulError),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Archive {
    /// The `archive` struct is an opaque type.
    archive: std::ptr::NonNull<archive>,
    filename: Option<CString>,
}

impl Archive {
    /// Attempts to open an archive in read-only mode.
    ///
    /// See the [`OpenOptions::open`] method for more details.
    ///
    /// # Errors
    ///
    /// This function will return an error if `path` does not already exist.
    /// Other errors may also be returned according to [`OpenOptions::open`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use archivers::{Archive, Result};
    ///
    /// fn main() -> Result<()> {
    ///     let mut ar = Archive::open("data/foo.tar")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Archive> {
        unsafe {
            let a = archive_read_new();
            archive_read_support_filter_all(a);
            archive_read_support_format_all(a);

            let path_c_str = path_to_cstring(path.as_ref())?;
            let path_c_ptr = path_c_str.as_ptr() as *const c_char;
            let r = archive_read_open_filename(a, path_c_ptr, 10240);

            if r == ARCHIVE_OK as i32 {
                Ok(Archive {
                    archive: NonNull::new_unchecked(a),
                    filename: Some(path_c_str),
                })
            } else {
                Err(Error::LibArchive(r as u32))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archive_create() {
        let path = "data/foo.tar";
        let ar = Archive::open(path).expect("File doesn't exist!");

        assert_eq!(path, ar.filename.unwrap().to_str().unwrap());
    }
}

#[cfg(unix)]
fn path_to_cstring(path: &Path) -> Result<CString> {
    // The `as_bytes` method exists only on Unix-like systems.
    use std::os::unix::ffi::OsStrExt;
    Ok(CString::new(path.as_os_str().as_bytes())?)
}

#[cfg(windows)]
fn path_to_cstring(path: &Path) -> Result<CString> {
    // Try to convert to UTF-8. If this fails, `archivers` can't handle the path
    // anyway.
    match path.to_str() {
        Some(s) => Ok(CString::new(s)?),
        None => {
            let message = format!("Couldn't convert path '{}' to UTF-8", path.display());
            Err(message.into())
        }
    }
}
