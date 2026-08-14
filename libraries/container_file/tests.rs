#![allow(clippy::unwrap_used, clippy::indexing_slicing)] // a slice that is the wrong length is a test failure
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::libraries::container_file::{pack, unpack, ContainerFormat};

    const FORMAT: ContainerFormat = ContainerFormat {
        magic: b"TESTFILE",
        version: 3,
        version_noun: "format version",
        no_magic_message: "this is not one of ours",
    };

    fn sections() -> HashMap<String, Vec<u8>> {
        let mut sections = HashMap::new();
        sections.insert("first".to_owned(), vec![1, 2, 3]);
        sections.insert("second".to_owned(), vec![4, 5]);
        sections
    }

    #[test]
    fn test_header_is_the_magic_then_the_version() {
        let header = FORMAT.header();

        assert_eq!(header.len(), FORMAT.header_len());
        assert_eq!(&header[..8], b"TESTFILE");
        assert_eq!(&header[8..], &3_u32.to_le_bytes());
    }

    #[test]
    fn test_round_trip() {
        let file = FORMAT.write(&sections()).unwrap();
        let read = FORMAT.read(&file).unwrap();

        assert_eq!(read.get("first"), Some(&vec![1, 2, 3]));
        assert_eq!(read.get("second"), Some(&vec![4, 5]));
    }

    #[test]
    fn test_the_body_follows_the_header() {
        let file = FORMAT.write(&sections()).unwrap();

        assert_eq!(&file[..FORMAT.header_len()], FORMAT.header().as_slice());
        assert_eq!(FORMAT.read_header(&file).unwrap(), &file[FORMAT.header_len()..]);
    }

    /// A file that does not start with the magic usually predates the header rather than
    /// being some other kind of file, so the owner supplies the wording.
    #[test]
    fn test_a_file_without_the_magic_says_what_the_owner_said() {
        let error = FORMAT.read(&[0; 64]).unwrap_err().to_string();

        assert_eq!(error, "this is not one of ours");
    }

    /// The whole point: a file this build cannot read is *named*, rather than handed to a
    /// decoder that reports a decode error about bytes it was never going to understand.
    #[test]
    fn test_a_wrong_version_names_both_versions() {
        let older = ContainerFormat { version: 2, ..FORMAT };
        let file = older.write(&sections()).unwrap();

        let error = FORMAT.read(&file).unwrap_err().to_string();

        assert!(error.contains("format version 2"), "should name the file's version: {error}");
        assert!(error.contains("format version 3"), "should name this build's version: {error}");
    }

    #[test]
    fn test_a_file_too_short_for_a_header_is_reported_as_such() {
        let error = FORMAT.read(b"TEST").unwrap_err().to_string();

        assert!(error.contains("too short"), "unexpected error: {error}");
    }

    /// A truncated body still gets past the header, which is correct - the header only
    /// promises that this build understands the *format*.
    #[test]
    fn test_a_truncated_body_fails_in_the_body() {
        let file = FORMAT.write(&sections()).unwrap();
        let truncated = &file[..file.len() - 3];

        FORMAT.read_header(truncated).unwrap();
        FORMAT.read(truncated).unwrap_err();
    }

    #[test]
    fn test_pack_round_trip() {
        let value = vec![7_u32; 1000];

        let packed = pack(&value).unwrap();

        assert!(packed.len() < value.len() * 4, "a thousand identical numbers should compress");
        assert_eq!(unpack::<Vec<u32>>(&packed).unwrap(), value);
    }

    #[test]
    fn test_unpack_rejects_garbage() {
        unpack::<Vec<u32>>(&[1, 2, 3, 4, 5]).unwrap_err();
    }
}
