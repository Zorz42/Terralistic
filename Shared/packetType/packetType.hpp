#ifndef packetType_hpp
#define packetType_hpp

#include <SFML/Network.hpp>

enum class PacketType {
    // blocks
    BLOCK_CHANGE, LIGHT_CHANGE, LIQUID_CHANGE,
    
    // entities
    ENTITY_VELOCITY,
    
    // players
    PLAYER_JOIN, PLAYER_LEAVE, PLAYER_VELOCITY_CHANGE,
    
    // view
    VIEW_SIZE_CHANGE, VIEW_POS_CHANGE,
    
    // items
    ITEM_CREATION, ITEM_DELETION,
    
    // inventory
    INVENTORY_CHANGE, INVENTORY_SWAP, HOTBAR_SELECTION, RECIPE_AVAILABILTY_CHANGE, CRAFT,
    
    // clicking
    RIGHT_CLICK, STARTED_BREAKING, STOPPED_BREAKING, BLOCK_PROGRESS_CHANGE,
    
    // miscellaneous
    KICK, CHAT,
};

sf::Packet& operator<<(sf::Packet& packet, PacketType packet_type);
sf::Packet& operator>>(sf::Packet& packet, PacketType& packet_type);

#endif
