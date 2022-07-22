#pragma once
#include "networking.hpp"

enum class ClientPacketType {
    _START,
    
    // players
    PLAYER_VELOCITY, PLAYER_MOVING_TYPE, PLAYER_JUMPED, ITEM_DROP, MAIN_PLAYER_POSITION, PLAYER_RESPAWN, PLAYER_SKIN,
    
    // inventory
    INVENTORY_SWAP, HOTBAR_SELECTION, CRAFT,
    
    // clicking
    RIGHT_CLICK, STARTED_BREAKING, STOPPED_BREAKING,
    
    // miscellaneous
    CHAT, PING,
    
    _END,
};

enum class ServerPacketType {
    _START,
    
    // blocks
    BLOCK, LIQUID, WALL, BLOCK_STARTED_BREAKING, BLOCK_STOPPED_BREAKING, WALL_STARTED_BREAKING, WALL_STOPPED_BREAKING, BLOCK_DATA_UPDATE,
    
    // entities
    ENTITY_VELOCITY, ENTITY_POSITION, ENTITY_DELETION,
    
    // players
    PLAYER_JOIN, PLAYER_MOVING_TYPE, PLAYER_JUMPED, MAIN_PLAYER_POSITION, PLAYER_SKIN,
    
    // items
    ITEM_CREATION, ITEM_COUNT_CHANGE,
    
    // inventory
    INVENTORY,

    // health
    HEALTH,

    // miscellaneous
    KICK, CHAT,

    // debug menu lines
    TPS, PING,
    
    _END,
};

enum class WelcomePacketType {
    _START, WELCOME, BLOCKS, WALLS, LIQUIDS, INVENTORY, TIME, HEALTH, _END,
};

Packet& operator<<(Packet& packet, ClientPacketType packet_type);
Packet& operator>>(Packet& packet, ClientPacketType& packet_type);

Packet& operator<<(Packet& packet, ServerPacketType packet_type);
Packet& operator>>(Packet& packet, ServerPacketType& packet_type);

Packet& operator<<(Packet& packet, WelcomePacketType packet_type);
Packet& operator>>(Packet& packet, WelcomePacketType& packet_type);

EXCEPTION_TYPE(PacketTypeError)
