#![allow(clippy::unwrap_used)] // tests assert on results directly
#[cfg(test)]
mod tests {
    use crate::libraries::serialization;
    use crate::shared::blocks::BlockChangePacket;
    use crate::shared::chat::ChatPacket;
    use crate::shared::packet::{Packet, WelcomeCompletePacket};
    use crate::shared::players::NamePacket;

    #[test]
    fn test_packet() {
        let packet = Packet::new(WelcomeCompletePacket).unwrap();
        assert!(packet.try_deserialize::<WelcomeCompletePacket>().is_some());
        assert!(packet.try_deserialize::<u32>().is_none());
    }

    #[test]
    fn test_packet_deserialize() {
        let packet = Packet::new(WelcomeCompletePacket).unwrap();
        packet.try_deserialize::<WelcomeCompletePacket>().unwrap();
        assert!(packet.try_deserialize::<u32>().is_none());
    }

    #[test]
    fn test_packet_serialize() {
        let packet = Packet::new(WelcomeCompletePacket).unwrap();
        let packet = packet.try_deserialize::<WelcomeCompletePacket>().unwrap();
        let packet = Packet::new(packet).unwrap();
        assert!(packet.try_deserialize::<WelcomeCompletePacket>().is_some());
        assert!(packet.try_deserialize::<u32>().is_none());
    }

    #[test]
    fn test_packet_serialize_deserialize() {
        let packet = Packet::new(WelcomeCompletePacket).unwrap();
        let packet = packet.try_deserialize::<WelcomeCompletePacket>().unwrap();
        let packet = Packet::new(packet).unwrap();
        let packet = packet.try_deserialize::<WelcomeCompletePacket>().unwrap();
        let packet = Packet::new(packet).unwrap();
        assert!(packet.try_deserialize::<WelcomeCompletePacket>().is_some());
        assert!(packet.try_deserialize::<u32>().is_none());
    }

    #[test]
    fn test_packet_to_data_from_data() {
        let packet = Packet::new(WelcomeCompletePacket).unwrap();
        let packet_data = serialization::serialize(&packet).unwrap();
        let packet = serialization::deserialize::<Packet>(&packet_data).unwrap();
        assert!(packet.try_deserialize::<WelcomeCompletePacket>().is_some());
        assert!(packet.try_deserialize::<u32>().is_none());
    }

    /// Two different packet types get different ids, which is the only thing stopping a
    /// receiver decoding one as the other.
    #[test]
    fn test_different_types_have_different_ids() {
        let name = Packet::new(NamePacket { name: "a".to_owned() }).unwrap();
        let chat = Packet::new(ChatPacket { message: "a".to_owned() }).unwrap();

        assert_ne!(name.id, chat.id, "two packet types collided on the same id");
    }

    /// The same type always hashes to the same id, whatever the payload.
    #[test]
    fn test_same_type_same_id_regardless_of_contents() {
        let a = Packet::new(ChatPacket { message: "short".to_owned() }).unwrap();
        let b = Packet::new(ChatPacket {
            message: "a considerably longer message".to_owned(),
        })
        .unwrap();

        assert_eq!(a.id, b.id);
    }

    /// Deserializing as the wrong type returns None rather than garbage, because the id is
    /// checked before the payload is touched.
    #[test]
    fn test_wrong_type_returns_none() {
        let packet = Packet::new(ChatPacket { message: "hi".to_owned() }).unwrap();

        assert!(packet.try_deserialize::<NamePacket>().is_none());
        assert!(packet.try_deserialize::<WelcomeCompletePacket>().is_none());
        assert!(packet.try_deserialize::<ChatPacket>().is_some());
    }

    #[test]
    fn test_payload_survives_the_round_trip() {
        let packet = Packet::new(BlockChangePacket {
            x: 12,
            y: -34,
            from_main_x: 1,
            from_main_y: 2,
            block: crate::shared::blocks::BlockId::undefined(),
            inventory: vec![],
        })
        .unwrap();

        let restored = packet.try_deserialize::<BlockChangePacket>().unwrap();
        assert_eq!(restored.x, 12);
        assert_eq!(restored.y, -34);
        assert_eq!(restored.from_main_x, 1);
        assert_eq!(restored.from_main_y, 2);
    }

    /// A packet is itself serializable, which is how it goes over the wire.
    #[test]
    fn test_packet_survives_being_sent() {
        let packet = Packet::new(ChatPacket { message: "over the wire".to_owned() }).unwrap();

        let bytes = serialization::serialize(&packet).unwrap();
        let received: Packet = serialization::deserialize(&bytes).unwrap();

        assert_eq!(received.id, packet.id);
        assert_eq!(received.try_deserialize::<ChatPacket>().unwrap().message, "over the wire");
    }
}
