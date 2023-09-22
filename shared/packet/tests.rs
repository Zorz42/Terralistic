#[cfg(test)]
mod tests {
    use crate::shared::packet::{Packet, WelcomeCompletePacket};

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
        let packet_data = bincode::serialize(&packet).unwrap();
        let packet = bincode::deserialize::<Packet>(&packet_data).unwrap();
        assert!(packet.try_deserialize::<WelcomeCompletePacket>().is_some());
        assert!(packet.try_deserialize::<u32>().is_none());
    }
}
