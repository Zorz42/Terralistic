#include "packetType.hpp"

sf::Packet& operator<<(sf::Packet& packet, ClientPacketType packet_type) {
    return packet << (unsigned char)packet_type;
}

sf::Packet& operator>>(sf::Packet& packet, ClientPacketType& packet_type) {
    unsigned char packet_type_char;
    packet >> packet_type_char;
    packet_type = (ClientPacketType)packet_type_char;
    return packet;
}

sf::Packet& operator<<(sf::Packet& packet, ServerPacketType packet_type) {
    return packet << (unsigned char)packet_type;
}

sf::Packet& operator>>(sf::Packet& packet, ServerPacketType& packet_type) {
    unsigned char packet_type_char;
    packet >> packet_type_char;
    packet_type = (ServerPacketType)packet_type_char;
    return packet;
}

sf::Packet& operator<<(sf::Packet& packet, WelcomePacketType packet_type) {
    return packet << (unsigned char)packet_type;
}

sf::Packet& operator>>(sf::Packet& packet, WelcomePacketType& packet_type) {
    unsigned char packet_type_char;
    packet >> packet_type_char;
    packet_type = (WelcomePacketType)packet_type_char;
    return packet;
}
