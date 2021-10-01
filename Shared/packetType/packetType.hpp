#pragma once

#include <SFML/Network.hpp>

enum class PacketType {
    // blocks
    BLOCK, LIQUID,
    
    // entities
    ENTITY_VELOCITY, ENTITY_POSITION, ENTITY_DELETION,
    
    // players
    PLAYER_JOIN, PLAYER_VELOCITY, PLAYER_MOVING_TYPE, PLAYER_JUMPED,
    
    // items
    ITEM_CREATION,
    
    // inventory
    INVENTORY, INVENTORY_SWAP, HOTBAR_SELECTION, RECIPE_AVAILABILTY_CHANGE, CRAFT,
    
    // clicking
    RIGHT_CLICK, STARTED_BREAKING, STOPPED_BREAKING, BLOCK_PROGRESS,
    
    // miscellaneous
    KICK, CHAT, WELCOME,
};

sf::Packet& operator<<(sf::Packet& packet, PacketType packet_type);
sf::Packet& operator>>(sf::Packet& packet, PacketType& packet_type);
