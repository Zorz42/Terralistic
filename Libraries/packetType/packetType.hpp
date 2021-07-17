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

sf::Packet& operator<<(sf::Packet& packet, PacketType packet_type);
sf::Packet& operator>>(sf::Packet& packet, PacketType& packet_type);

#endif /* packetType_hpp */
