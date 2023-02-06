use enet::Enet;
use once_cell::sync::Lazy;

pub static ENET_GLOBAL: Lazy<Enet> = Lazy::new(|| Enet::new().unwrap());
