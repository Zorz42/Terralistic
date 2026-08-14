use std::any::TypeId;
use std::hash::{Hash, Hasher};

use anyhow::Result;
use fnv::FnvHasher;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};

use crate::libraries::serialization;

/// This function returns a hash of a type. The hash is always the same for the same type
/// on every run of the program and on every machine and is unique for every type.
fn get_type_id<Type: 'static>() -> u64 {
    let mut hasher = FnvHasher::default();
    let type_id = TypeId::of::<Type>();
    type_id.hash(&mut hasher);
    hasher.finish()
}

/// A message on the wire: an id and some bytes.
///
/// The id is a hash of the rust type the bytes were serialized from, so **any serializable
/// type is a packet** - there is no registry to add to, no id to allocate, and no way for
/// two packet types to end up sharing a number. A receiver offers an incoming packet to each
/// type it knows about with `try_deserialize`, and only the right one answers.
///
/// The price of that is written down here because it is easy to be caught by:
///
/// - `TypeId` is not stable across compiler versions, so both ends must be built by the same
///   rustc. Nothing in here can detect that; a protocol version packet sent first is how a
///   caller makes it diagnosable.
/// - Renaming a packet struct silently changes its id, which is a wire break with no
///   compile error anywhere.
#[derive(Serialize, Deserialize)]
pub struct Packet {
    pub id: u64,
    pub data: Vec<u8>,
}

impl Packet {
    /// This function creates a new packet from a serializable object.
    pub fn new<T: serde::Serialize + 'static>(data: T) -> Result<Self> {
        let id = get_type_id::<T>();
        let data = serialization::serialize(&data)?;
        Ok(Self { id, data })
    }

    /// This function deserializes the data in the packet to the type that the packet was created from.
    /// If the type of the packet is not the same as the type that you are trying to deserialize to,
    /// it will return None.
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
