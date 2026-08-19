use crate::libraries::fixed::Fixed;
use crate::libraries::registry::{NamedEntry, RegistryEntry};
use crate::shared::liquids::LiquidId;

/// `LiquidType` holds all information about a type of a liquid.
#[derive(Clone)]
pub struct LiquidType {
    pub(super) id: LiquidId,
    /// How many milliseconds pass between two flow steps of this liquid. Bigger is slower,
    /// and 0 means it never flows at all - which is what the built in empty liquid is.
    pub flow_time: i32,
    /// How fast an entity moves through this liquid, as a fraction of how fast it moves
    /// through air. 1.0 is a liquid that does not slow anything down.
    pub speed_multiplier: Fixed,
    pub name: String,
}

impl LiquidType {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: LiquidId::undefined(),
            flow_time: 100,
            speed_multiplier: Fixed::ONE,
            name: String::new(),
        }
    }

    #[must_use]
    pub const fn get_id(&self) -> LiquidId {
        self.id
    }
}

impl RegistryEntry<LiquidId> for LiquidType {
    fn set_id(&mut self, id: LiquidId) {
        self.id = id;
    }
}

impl NamedEntry for LiquidType {
    fn get_name(&self) -> &str {
        &self.name
    }
}
