use std::any::TypeId;
use std::hash::{Hash, Hasher};

use anyhow::Result;
use fnv::FnvHasher;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};

use crate::libraries::serialization;

/// A hash of a type: the same on every run and every machine, and unique per type.
fn get_type_id<Type: 'static>() -> u64 {
    let mut hasher = FnvHasher::default();
    let type_id = TypeId::of::<Type>();
    type_id.hash(&mut hasher);
    hasher.finish()
}

/// A message on the wire: an id and some bytes.
///
/// The id is a hash of the rust type the bytes came from, so **any serializable type is a
/// packet**: no registry, no allocated ids, and no way for two types to share a number. A
/// receiver offers each type it knows to `try_deserialize`, and only the right one answers. The
/// price is worth knowing:
///
/// - `TypeId` is not stable across compiler versions, so both ends need the same rustc.
///   Nothing here detects that; a version packet sent first is how a caller diagnoses it.
/// - Renaming a packet struct silently changes its id - a wire break with no compile error.
#[derive(Serialize, Deserialize)]
pub struct Packet {
    pub id: u64,
    pub data: Vec<u8>,
}

impl Packet {
    /// Serializes a value into a packet tagged with its type.
    pub fn new<T: serde::Serialize + 'static>(data: T) -> Result<Self> {
        let id = get_type_id::<T>();
        let data = serialization::serialize(&data)?;
        Ok(Self { id, data })
    }

    /// The packet's contents as a `T`, or `None` if it carries something else.
    #[must_use]
    pub fn try_deserialize<T: DeserializeOwned + 'static>(&self) -> Option<T> {
        if self.id == get_type_id::<T>() {
            serialization::deserialize(&self.data).map_or_else(|_| None, |data| Some(data))
        } else {
            None
        }
    }

    /// Whether this packet carries a `T`, without paying to decode it.
    #[must_use]
    pub fn is<T: 'static>(&self) -> bool {
        self.id == get_type_id::<T>()
    }
}
