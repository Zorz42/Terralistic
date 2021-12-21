#pragma once
#include <SFML/Network.hpp>

enum class ClientPacketType {
    _START,
    
    // players
    PLAYER_VELOCITY, PLAYER_MOVING_TYPE, PLAYER_JUMPED, ITEM_DROP,
    
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
    BLOCK, LIQUID, STARTED_BREAKING, STOPPED_BREAKING,
    
    // entities
    ENTITY_VELOCITY, ENTITY_POSITION, ENTITY_DELETION,
    
    // players
    PLAYER_JOIN, PLAYER_VELOCITY, PLAYER_MOVING_TYPE, PLAYER_JUMPED,
    
    // items
    ITEM_CREATION,
    
    // inventory
    INVENTORY,
    
    // miscellaneous
    KICK, CHAT,
    
    _END,
};

enum class WelcomePacketType {
    _START, WELCOME, BLOCKS, LIQUIDS, INVENTORY, TIME, _END,
};

sf::Packet& operator<<(sf::Packet& packet, ClientPacketType packet_type);
sf::Packet& operator>>(sf::Packet& packet, ClientPacketType& packet_type);

sf::Packet& operator<<(sf::Packet& packet, ServerPacketType packet_type);
sf::Packet& operator>>(sf::Packet& packet, ServerPacketType& packet_type);

sf::Packet& operator<<(sf::Packet& packet, WelcomePacketType packet_type);
sf::Packet& operator>>(sf::Packet& packet, WelcomePacketType& packet_type);
