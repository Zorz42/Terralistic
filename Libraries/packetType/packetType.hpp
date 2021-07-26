#ifndef packetType_hpp
#define packetType_hpp

#include <SFML/Network.hpp>

enum class PacketType {CHUNK, BLOCK_CHANGE, LIGHT_CHANGE, LIQUID_CHANGE, PLAYER_JOIN, PLAYER_QUIT, PLAYER_MOVEMENT, ITEM_CREATION, ITEM_DELETION, ITEM_MOVEMENT, INVENTORY_CHANGE, INVENTORY_SWAP, HOTBAR_SELECTION, RIGHT_CLICK, STARTED_BREAKING, STOPPED_BREAKING, BLOCK_PROGRESS_CHANGE, SPAWN_POS, VIEW_SIZE_CHANGE, KICK, CHAT, VIEW_POS_CHANGE};

sf::Packet& operator<<(sf::Packet& packet, PacketType packet_type);
sf::Packet& operator>>(sf::Packet& packet, PacketType& packet_type);

#endif
