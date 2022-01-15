use libarchive_sys::*;

mod archive;
pub use archive::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_archive_content() {
        let version = ARCHIVE_VERSION_NUMBER;

        assert!(
            version >= 3000000,
            "You are using a version of `libarchive` earlier than version 3"
        );
    }
}
