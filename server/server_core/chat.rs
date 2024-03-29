use anyhow::Result;

use crate::libraries::events::Event;
use crate::server::server_core::{
    networking::{PacketFromClientEvent, SendTarget, ServerNetworking},
    print_to_console,
};
use crate::shared::chat::ChatPacket;
use crate::shared::packet::Packet;

pub fn server_chat_on_event(event: &Event, networking: &mut ServerNetworking) -> Result<()> {
    if let Some(event) = event.downcast::<PacketFromClientEvent>() {
        if let Some(packet) = event.packet.try_deserialize::<ChatPacket>() {
            if packet.message.starts_with('/') {
                return Ok(());
            }

            let mut name = networking.get_connection_name(&event.conn);
            if name == "_" {
                name = "Player".to_owned();
            }

            let message = format!("{}: {}", name, packet.message);
            print_to_console(&message, 0);
            networking.send_packet(&Packet::new(ChatPacket { message })?, SendTarget::All)?;
        }
    }
    Ok(())
}
