use std::marker::PhantomData;

use anyhow::{anyhow, bail, Result};

/// A typed handle into a `Registry`, implemented by a newtype around whatever integer it is
/// sent as - so a `BlockId` will not compile where a `WallId` is wanted, both being `i8`.
///
/// An id is a **handle, not a position**: `index` is an implementation detail of the lookup,
/// and nothing outside a registry should lay anything out by it.
pub trait RegistryId: Copy + Eq {
    /// What these ids identify, for error messages: "block type", "recipe".
    const KIND: &'static str;

    /// The handle for the entry at `index`. Called once per registration.
    fn from_index(index: usize) -> Self;

    /// Where to look for this handle's entry, or `None` for the "undefined" value every one
    /// of these newtypes has.
    fn index(self) -> Option<usize>;
}

/// An entry told its own handle when it is registered, because entries are handed around away
/// from the registry - a block type reaches rendering, which has no registry to ask.
pub trait RegistryEntry<Id: RegistryId> {
    fn set_id(&mut self, id: Id);
}

/// An entry that can be looked up by name. Separate from `RegistryEntry` because not every
/// registry has names to look up by - recipes are registered and referred to only by id.
pub trait NamedEntry {
    fn get_name(&self) -> &str;
}

/// Register a value, get a typed handle back.
///
/// Handles are handed out in registration order and never reused, entries are never removed, and
/// every lookup is checked - an unknown handle names what it failed to find rather than panicking
/// or reading whatever the arithmetic hit.
///
/// **Not in scope**: persistence. A registry is rebuilt from whatever defines its entries, and
/// saves refer to them by *name* - saving handles would freeze registration order into the
/// file. Lookup by name is a linear scan, right for tens of entries consulted at load time and
/// wrong for thousands consulted per frame.
pub struct Registry<Id, T> {
    entries: Vec<T>,
    marker: PhantomData<Id>,
}

impl<Id, T> Registry<Id, T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
            marker: PhantomData,
        }
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.entries.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.entries.iter()
    }
}

impl<'reg, Id, T> IntoIterator for &'reg Registry<Id, T> {
    type Item = &'reg T;
    type IntoIter = std::slice::Iter<'reg, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<Id: RegistryId, T> Registry<Id, T> {
    pub fn get(&self, id: Id) -> Result<&T> {
        let index = id.index().ok_or_else(|| anyhow!("no {} for an undefined id", Id::KIND))?;
        self.entries.get(index).ok_or_else(|| anyhow!("no {} with id {index}", Id::KIND))
    }

    pub fn get_mut(&mut self, id: Id) -> Result<&mut T> {
        let index = id.index().ok_or_else(|| anyhow!("no {} for an undefined id", Id::KIND))?;
        self.entries.get_mut(index).ok_or_else(|| anyhow!("no {} with id {index}", Id::KIND))
    }

    /// Every handle this registry has handed out, in registration order.
    #[must_use]
    pub fn ids(&self) -> Vec<Id> {
        (0..self.entries.len()).map(Id::from_index).collect()
    }
}

impl<Id: RegistryId, T: RegistryEntry<Id>> Registry<Id, T> {
    /// Adds an entry and returns its handle, stamping it in first so an entry handed out on
    /// its own still knows what it is.
    pub fn register(&mut self, mut entry: T) -> Id {
        let id = Id::from_index(self.entries.len());
        entry.set_id(id);
        self.entries.push(entry);
        id
    }
}

impl<Id: RegistryId, T: NamedEntry> Registry<Id, T> {
    pub fn get_by_name(&self, name: &str) -> Result<&T> {
        match self.entries.iter().find(|entry| entry.get_name() == name) {
            Some(entry) => Ok(entry),
            None => bail!("there is no {} called \"{name}\"", Id::KIND),
        }
    }

    pub fn get_id_by_name(&self, name: &str) -> Result<Id> {
        for (index, entry) in self.entries.iter().enumerate() {
            if entry.get_name() == name {
                return Ok(Id::from_index(index));
            }
        }
        bail!("there is no {} called \"{name}\"", Id::KIND)
    }
}
