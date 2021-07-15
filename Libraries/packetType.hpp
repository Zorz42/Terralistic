//
//  packetType.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 14/07/2021.
//

#ifndef packetType_hpp
#define packetType_hpp

#include <SFML/Network.hpp>

enum class PacketType {DISCONNECT, CHUNK, BLOCK_CHANGE, PLAYER_JOIN, PLAYER_QUIT, PLAYER_MOVEMENT, ITEM_CREATION, ITEM_DELETION, ITEM_MOVEMENT, INVENTORY_CHANGE, INVENTORY_SWAP, HOTBAR_SELECTION, RIGHT_CLICK, STARTED_BREAKING, STOPPED_BREAKING, BLOCK_PROGRESS_CHANGE, SPAWN_POS, VIEW_SIZE_CHANGE, KICK, CHAT};

inline sf::Packet& operator<<(sf::Packet& packet, PacketType packet_type) {
    return packet << (unsigned char)packet_type;
}

inline sf::Packet& operator>>(sf::Packet& packet, PacketType& packet_type) {
    unsigned char packet_type_char;
    packet >> packet_type_char;
    packet_type = (PacketType)packet_type_char;
    return packet;
}

#endif /* packetType_hpp */
