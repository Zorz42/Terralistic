use std::collections::HashMap;

use anyhow::{bail, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::libraries::serialization;

/// One kind of container file: what it starts with, and which version this build reads.
#[derive(Clone, Copy)]
pub struct ContainerFormat {
    /// What every file of this kind starts with. Fixed width, untouched by any serializer.
    pub magic: &'static [u8],
    /// Bumped by hand whenever the contents change shape.
    pub version: u32,
    /// What to call the version in an error, e.g. "save version" - the owner's word for it is
    /// more use to a reader than "version 4".
    pub version_noun: &'static str,
    /// What to say when a file does not start with `magic`, which usually means it predates
    /// the header rather than being the wrong kind of file.
    pub no_magic_message: &'static str,
}

impl ContainerFormat {
    /// Magic plus a little endian `u32`.
    #[must_use]
    pub const fn header_len(&self) -> usize {
        self.magic.len() + 4
    }

    /// The bytes a file of this kind starts with.
    #[must_use]
    pub fn header(&self) -> Vec<u8> {
        let mut header = Vec::with_capacity(self.header_len());
        header.extend_from_slice(self.magic);
        header.extend_from_slice(&self.version.to_le_bytes());
        header
    }

    /// Checks a file's header and returns the body after it. Every rejection names what is
    /// wrong, which is the entire reason the header exists.
    pub fn read_header<'file>(&self, file: &'file [u8]) -> Result<&'file [u8]> {
        let Some((header, body)) = file.split_at_checked(self.header_len()) else {
            bail!("this file is too short to have a header - it is {} bytes", file.len());
        };
        let (magic, version) = header.split_at(self.magic.len());

        if magic != self.magic {
            bail!("{}", self.no_magic_message);
        }

        let version = u32::from_le_bytes(version.try_into().unwrap_or([0; 4]));
        if version != self.version {
            bail!("this file is {} {version}, but this build reads {} {}", self.version_noun, self.version_noun, self.version);
        }
        Ok(body)
    }

    /// Writes the header followed by the sections.
    pub fn write(&self, sections: &HashMap<String, Vec<u8>>) -> Result<Vec<u8>> {
        let mut file = self.header();
        serialization::serialize_into(&mut file, sections)?;
        Ok(file)
    }

    /// Checks the header and decodes the sections.
    pub fn read(&self, file: &[u8]) -> Result<HashMap<String, Vec<u8>>> {
        serialization::deserialize(self.read_header(file)?)
    }
}

/// Serializes and compresses a value, for a section big enough to be worth it.
///
/// Sections are opaque bytes, so compressing one is the owner's choice - a dense grid pays for
/// itself many times over, a handful of records does not. Here so the choice is spelled the same
/// way.
pub fn pack<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    Ok(snap::raw::Encoder::new().compress_vec(&serialization::serialize(value)?)?)
}

/// The other half of `pack`.
pub fn unpack<T: DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    serialization::deserialize(&snap::raw::Decoder::new().decompress_vec(bytes)?)
}
