#include "packetType.hpp"
#include "exception.hpp"

Packet& operator<<(Packet& packet, ClientPacketType packet_type) {
    if(packet_type <= ClientPacketType::_START || packet_type >= ClientPacketType::_END)
        throw Exception("Invalid client packet value type.");
    return packet << (int)packet_type;
}

Packet& operator>>(Packet& packet, ClientPacketType& packet_type) {
    int packet_type_char;
    packet >> packet_type_char;
    packet_type = (ClientPacketType)packet_type_char;
    if(packet_type <= ClientPacketType::_START || packet_type >= ClientPacketType::_END)
        throw Exception("Invalid client packet value type.");
    return packet;
}

Packet& operator<<(Packet& packet, ServerPacketType packet_type) {
    if(packet_type <= ServerPacketType::_START || packet_type >= ServerPacketType::_END)
        throw Exception("Invalid server packet value type.");
    return packet << (int)packet_type;
}

Packet& operator>>(Packet& packet, ServerPacketType& packet_type) {
    int packet_type_char;
    packet >> packet_type_char;
    packet_type = (ServerPacketType)packet_type_char;
    if(packet_type <= ServerPacketType::_START || packet_type >= ServerPacketType::_END)
        throw Exception("Invalid server packet value type.");
    return packet;
}

Packet& operator<<(Packet& packet, WelcomePacketType packet_type) {
    if(packet_type <= WelcomePacketType::_START || packet_type >= WelcomePacketType::_END)
        throw Exception("Invalid welcome packet value type.");
    return packet << (int)packet_type;
}

Packet& operator>>(Packet& packet, WelcomePacketType& packet_type) {
    int packet_type_char;
    packet >> packet_type_char;
    packet_type = (WelcomePacketType)packet_type_char;
    if(packet_type <= WelcomePacketType::_START || packet_type >= WelcomePacketType::_END)
        throw Exception("Invalid welcome packet value type.");
    return packet;
}
