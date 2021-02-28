//
//  blockEngine.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 12/01/2021.
//

#define FILENAME blockEngine
#define NAMESPACE blockEngine
#include "core.hpp"

#include <chrono>
#include "main.hpp"
#include "blockEngine.hpp"
#include "print.hpp"

blockEngine::block& blockEngine::getBlock(unsigned short x, unsigned short y) {
    return world[y * world_width + x];
}

void grass_block_leftClickEvent(blockEngine::block* block, unsigned short x, unsigned short y, playerHandler::player* player) {
    blockEngine::blockChange(x, y, blockEngine::DIRT);
}

void air_rightClickEvent(blockEngine::block* block, unsigned short x, unsigned short y, playerHandler::player* player) {
    blockEngine::blockType type = player->inventory.inventory[player->inventory.selected_slot].getUniqueItem().places;
    if(type != blockEngine::AIR && player->inventory.inventory[player->inventory.selected_slot].decreaseStack(1)) {
        packets::packet item_loss_packet(packets::INVENTORY_CHANGE);
        item_loss_packet << (unsigned char)player->inventory.inventory[player->inventory.selected_slot].item_id << (unsigned short)player->inventory.inventory[player->inventory.selected_slot].getStack() << (char)player->inventory.selected_slot;
        player->conn->sendPacket(item_loss_packet);
        blockEngine::blockChange(x, y, type);
        blockEngine::getBlock(x, y).update(x, y);
        blockEngine::updateNearestBlocks(x, y);
    }
}

void air_leftClickEvent(blockEngine::block* block, unsigned short x, unsigned short y, playerHandler::player* player) {}

INIT_SCRIPT
    using namespace blockEngine;
    unique_blocks = {
        uniqueBlock("air",         /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/itemEngine::NOTHING,     /*break_time*/1000),
        uniqueBlock("dirt",        /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/itemEngine::DIRT,        /*break_time*/1000),
        uniqueBlock("stone_block", /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/itemEngine::STONE_BLOCK, /*break_time*/1000),
        uniqueBlock("grass_block", /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/itemEngine::NOTHING,     /*break_time*/1000),
        uniqueBlock("stone",       /*ghost*/true,  /*only_on_floor*/true,   /*transparent*/true,  /*drop*/itemEngine::STONE,       /*break_time*/1000),
        uniqueBlock("wood",        /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/itemEngine::NOTHING,     /*break_time*/1000),
        uniqueBlock("leaves",      /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/itemEngine::NOTHING,     /*break_time*/1000),
    };
    
    unique_blocks[GRASS_BLOCK].leftClickEvent = &grass_block_leftClickEvent;
    unique_blocks[AIR].rightClickEvent = &air_rightClickEvent;
    unique_blocks[AIR].leftClickEvent = &air_leftClickEvent;

    world_height = 1200;
    world_width = 4400;
    world = new block[world_width * world_height];
INIT_SCRIPT_END

void blockEngine::updateNearestBlocks(unsigned short x, unsigned short y) {
    block* neighbors[4] = {nullptr, nullptr, nullptr, nullptr};
    unsigned short x_[4] = {(unsigned short)(x - 1), (unsigned short)(x + 1), x, x}, y_[4] = {y, y, (unsigned short)(y - 1), (unsigned short)(y + 1)};
    if(x != 0)
        neighbors[0] = &getBlock(x - 1, y);
    if(x != blockEngine::world_width - 1)
        neighbors[1] = &getBlock(x + 1, y);
    if(y != 0)
        neighbors[2] = &getBlock(x, y - 1);
    if(y != blockEngine::world_height - 1)
        neighbors[3] = &getBlock(x, y + 1);
    for(int i = 0; i < 4; i++)
        if(neighbors[i] != nullptr)
            neighbors[i]->update(x_[i], y_[i]);
}

void blockEngine::blockChange(unsigned short x, unsigned short y, blockType type, networking::connection* conn) {
    blockEngine::getBlock(x, y).block_id = type;
    packets::packet packet(packets::BLOCK_CHANGE);
    packet << x << y << (unsigned char) type;
    networking::sendToEveryone(packet, conn);
    blockEngine::updateNearestBlocks(x, y);
}

PACKET_LISTENER(packets::STARTED_BREAKING)
    unsigned short y = packet.getUShort(), x = packet.getUShort();
    playerHandler::player* player = playerHandler::getPlayerByConnection(&connection);
    player->breaking_x = x;
    player->breaking_y = y;
    player->breaking = true;
PACKET_LISTENER_END

PACKET_LISTENER(packets::STOPPED_BREAKING)
    playerHandler::getPlayerByConnection(&connection)->breaking = false;
PACKET_LISTENER_END

PACKET_LISTENER(packets::RIGHT_CLICK)
    unsigned short y = packet.getUShort(), x = packet.getUShort();
    blockEngine::block* block = &blockEngine::getBlock(x, y);
    if(block->getUniqueBlock().rightClickEvent)
        block->getUniqueBlock().rightClickEvent(block, x, y, playerHandler::getPlayerByConnection(&connection));
PACKET_LISTENER_END

PACKET_LISTENER(packets::CHUNK)
    packets::packet chunk_packet(packets::CHUNK);
    unsigned short x = packet.getUShort(), y = packet.getUShort();
    for(int i = 0; i < 16 * 16; i++)
        chunk_packet << (char)blockEngine::getBlock((x << 4) + 15 - i % 16, (y << 4) + 15 - i / 16).block_id;
    chunk_packet << y << x;
    connection.sendPacket(chunk_packet);
PACKET_LISTENER_END

void leftClickEvent(unsigned short x, unsigned short y, networking::connection& connection) {
    blockEngine::block* block = &blockEngine::getBlock(x, y);
    if(block->getUniqueBlock().leftClickEvent)
        block->getUniqueBlock().leftClickEvent(block, x, y, playerHandler::getPlayerByConnection(&connection));
    else {
        char prev = (float)block->break_progress / (float)block->getUniqueBlock().break_time * 9.0f, curr;
        block->break_progress += main_::frame_length;
        curr = (float)block->break_progress / (float)block->getUniqueBlock().break_time * 9.0f;
        if(curr != prev) {
            packets::packet packet(packets::BLOCK_BREAK_PROGRESS_CHANGE);
            packet << y << x << block->break_progress;
            networking::sendToEveryone(packet);
        }
        if(block->break_progress >= block->getUniqueBlock().break_time) {
            itemEngine::spawnItem(blockEngine::getBlock(x, y).getUniqueBlock().drop, x * BLOCK_WIDTH, y * BLOCK_WIDTH);
            blockEngine::blockChange(x, y, blockEngine::AIR);
        }
    }
}

void blockEngine::updatePlayersBreaking() {
    for(playerHandler::player& player : playerHandler::players)
        if(player.breaking)
            leftClickEvent(player.breaking_x, player.breaking_y, *player.conn);
}
