use once_cell::sync::Lazy;
use enet::Enet;

pub static ENET_GLOBAL: Lazy<Enet> = Lazy::new(|| Enet::new().unwrap());