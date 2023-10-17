use std::any::TypeId;
use std::hash::{Hash, Hasher};

use anyhow::Result;
use fnv::FnvHasher;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};

/// Packet has an id and data. Id is determined by the hash of the object
/// that data is serialized from. You can serialize and deserialize it.
#[derive(Serialize, Deserialize)]
pub struct Packet {
    pub id: u64,
    pub data: Vec<u8>,
}

/// This function returns a hash of a type. The hash is always the same for the same type
/// on every run of the program and on every machine and is unique for every type.
fn get_type_id<Type: 'static>() -> u64 {
    let mut hasher = FnvHasher::default();
    let type_id = TypeId::of::<Type>();
    type_id.hash(&mut hasher);
    hasher.finish()
}

impl Packet {
    /// This function creates a new packet from a serializable object.
    pub fn new<T: serde::Serialize + 'static>(data: T) -> Result<Self> {
        let id = get_type_id::<T>();
        let data = bincode::serialize(&data)?;
        Ok(Self { id, data })
    }

    /// This function deserializes the data in the packet to the type that the packet was created from.
    /// If the type of the packet is not the same as the type that you are trying to deserialize to,
    /// it will return None.
    #[must_use]
    pub fn try_deserialize<T: DeserializeOwned + 'static>(&self) -> Option<T> {
        if self.id == get_type_id::<T>() {
            bincode::deserialize(&self.data).map_or_else(|_| None, |data| Some(data))
        } else {
            None
        }
    }
}

/// This packet is sent when all the welcome packets have been sent.
#[derive(Serialize, Deserialize)]
pub struct WelcomeCompletePacket;
