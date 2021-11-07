#include "packetType.hpp"

sf::Packet& operator<<(sf::Packet& packet, ClientPacketType packet_type) {
    if(packet_type <= ClientPacketType::_START || packet_type >= ClientPacketType::_END)
        throw PacketValueException();
    return packet << (int)packet_type;
}

sf::Packet& operator>>(sf::Packet& packet, ClientPacketType& packet_type) {
    int packet_type_char;
    packet >> packet_type_char;
    packet_type = (ClientPacketType)packet_type_char;
    if(packet_type <= ClientPacketType::_START || packet_type >= ClientPacketType::_END)
        throw PacketValueException();
    return packet;
}

sf::Packet& operator<<(sf::Packet& packet, ServerPacketType packet_type) {
    if(packet_type <= ServerPacketType::_START || packet_type >= ServerPacketType::_END)
        throw PacketValueException();
    return packet << (int)packet_type;
}

sf::Packet& operator>>(sf::Packet& packet, ServerPacketType& packet_type) {
    int packet_type_char;
    packet >> packet_type_char;
    packet_type = (ServerPacketType)packet_type_char;
    if(packet_type <= ServerPacketType::_START || packet_type >= ServerPacketType::_END)
        throw PacketValueException();
    return packet;
}

sf::Packet& operator<<(sf::Packet& packet, WelcomePacketType packet_type) {
    if(packet_type <= WelcomePacketType::_START || packet_type >= WelcomePacketType::_END)
        throw PacketValueException();
    return packet << (int)packet_type;
}

sf::Packet& operator>>(sf::Packet& packet, WelcomePacketType& packet_type) {
    int packet_type_char;
    packet >> packet_type_char;
    packet_type = (WelcomePacketType)packet_type_char;
    if(packet_type <= WelcomePacketType::_START || packet_type >= WelcomePacketType::_END)
        throw PacketValueException();
    return packet;
}
