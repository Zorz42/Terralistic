#pragma once
#include <SFML/Network.hpp>

enum class ClientPacketType {
    _START,
    
    // players
    PLAYER_VELOCITY, PLAYER_MOVING_TYPE, PLAYER_JUMPED, ITEM_DROP, MAIN_PLAYER_POSITION,
    
    // inventory
    INVENTORY_SWAP, HOTBAR_SELECTION, CRAFT,
    
    // clicking
    RIGHT_CLICK, STARTED_BREAKING, STOPPED_BREAKING,
    
    // miscellaneous
    CHAT,
    
    _END,
};

enum class ServerPacketType {
    _START,
    
    // blocks
    BLOCK, LIQUID, WALL, BLOCK_STARTED_BREAKING, BLOCK_STOPPED_BREAKING, WALL_STARTED_BREAKING, WALL_STOPPED_BREAKING,
    
    // entities
    ENTITY_VELOCITY, ENTITY_POSITION, ENTITY_DELETION,
    
    // players
    PLAYER_JOIN, PLAYER_MOVING_TYPE, PLAYER_JUMPED, MAIN_PLAYER_POSITION,
    
    // items
    ITEM_CREATION,
    
    // inventory
    INVENTORY,

    // health
    HEALTH,

    // miscellaneous
    KICK, CHAT,
    
    _END,
};

enum class WelcomePacketType {
    _START, WELCOME, BLOCKS, WALLS, LIQUIDS, INVENTORY, TIME, HEALTH, _END,
};

sf::Packet& operator<<(sf::Packet& packet, ClientPacketType packet_type);
sf::Packet& operator>>(sf::Packet& packet, ClientPacketType& packet_type);

sf::Packet& operator<<(sf::Packet& packet, ServerPacketType packet_type);
sf::Packet& operator>>(sf::Packet& packet, ServerPacketType& packet_type);

sf::Packet& operator<<(sf::Packet& packet, WelcomePacketType packet_type);
sf::Packet& operator>>(sf::Packet& packet, WelcomePacketType& packet_type);
