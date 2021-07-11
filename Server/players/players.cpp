//
//  players.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 22/06/2021.
//

#include "players.hpp"
#include "blocks.hpp"
#include <filesystem>
#include <fstream>

players::players(serverNetworkingManager* manager_, blocks* parent_blocks_, items* parent_items_) : parent_blocks(parent_blocks_), parent_items(parent_items_), serverPacketListener(manager_), manager(manager_) {
    listening_to = {PacketType::STARTED_BREAKING, PacketType::STOPPED_BREAKING, PacketType::RIGHT_CLICK, PacketType::CHUNK, PacketType::VIEW_SIZE_CHANGE, PacketType::PLAYER_MOVEMENT, PacketType::PLAYER_JOIN, PacketType::DISCONNECT, PacketType::INVENTORY_SWAP, PacketType::HOTBAR_SELECTION, PacketType::CHAT};
    custom_block_events[(int)blockType::WOOD].onBreak = [](blocks* server_blocks, players* server_players, block* this_block) {
        block blocks[] = {server_blocks->getBlock(this_block->getX(), this_block->getY() - 1), server_blocks->getBlock(this_block->getX() + 1, this_block->getY()), server_blocks->getBlock(this_block->getX() - 1, this_block->getY())};
        for(block& i : blocks)
            if(i.getType() == blockType::WOOD || i.getType() == blockType::LEAVES)
                server_players->breakBlock(&i);
    };

    custom_block_events[(int)blockType::LEAVES].onBreak = custom_block_events[(int)blockType::WOOD].onBreak;

    custom_block_events[(int)blockType::GRASS_BLOCK].onLeftClick = [](block* this_block, player* peer) {
        this_block->setType(blockType::DIRT);
    };

    custom_block_events[(int)blockType::AIR].onRightClick = [](block* this_block, player* peer) {
        blockType type = peer->player_inventory.getSelectedSlot()->getUniqueItem().places;
        if(type != blockType::AIR && peer->player_inventory.inventory_arr[peer->player_inventory.selected_slot].decreaseStack(1)) {
            this_block->setType(type);
            this_block->update();
        }
    };

    custom_block_events[(int)blockType::SNOWY_GRASS_BLOCK].onLeftClick = custom_block_events[(int)blockType::GRASS_BLOCK].onLeftClick;
}

players::~players() {
    for(player* i : all_players)
        delete i;
}

player* players::getPlayerByConnection(connection* conn) {
    for(player* player : online_players)
        if(player->conn == conn)
            return player;
    return nullptr;
}

player* players::getPlayerByName(const std::string& name) {
    static unsigned int curr_id = 0;
    for(player* player : all_players)
        if(player->name == name)
            return player;
    all_players.emplace_back(new player(curr_id++));
    player* curr_player = all_players.back();
    curr_player->y = parent_blocks->getSpawnY() - BLOCK_WIDTH * 2;
    curr_player->x = parent_blocks->getSpawnX();
    curr_player->name = name;
    return curr_player;
}

void players::updatePlayersBreaking(unsigned short tick_length) {
    for(player* player : online_players)
        if(player->breaking)
            leftClickEvent(parent_blocks->getBlock(player->breaking_x, player->breaking_y), *player->conn, tick_length);
}

void players::lookForItems() {
    for(const item& i : parent_items->getItems())
        for(player* player : online_players)
            if(abs(i.x / 100 + BLOCK_WIDTH / 2  - player->x - 14) < 50 && abs(i.y / 100 + BLOCK_WIDTH / 2 - player->y - 25) < 50)
                if(player->player_inventory.addItem(i.getItemId(), 1) != -1)
                    parent_items->destroyItem(i);
}

void players::updateBlocks() {
    bool finished = false;
    while(!finished) {
        finished = true;
        for(player* player : online_players) {
            int start_x = player->x / 16 - player->sight_width / 2 - 20,
                start_y = player->y / 16 - player->sight_height / 2 - 20,
                end_x = player->x / 16 + player->sight_width / 2 + 20,
                end_y = player->y / 16 + player->sight_height / 2 + 20;
            if(start_x < 0)
                start_x = 0;
            if(start_y < 0)
                start_y = 0;
            if(end_y > parent_blocks->getHeight())
                end_y = parent_blocks->getHeight();
            if(end_x > parent_blocks->getWidth())
                end_x = parent_blocks->getWidth();
            for(unsigned short x = start_x; x < end_x; x++)
                for(unsigned short y = end_y - 1; y >= start_y; y--) {
                    block curr_block = parent_blocks->getBlock(x, y);
                    if(curr_block.hasScheduledLightUpdate()) {
                        curr_block.lightUpdate();
                        finished = false;
                    }
                    if(curr_block.getLiquidType() != liquidType::EMPTY && curr_block.canUpdateLiquid()) {
                        curr_block.liquidUpdate();
                        finished = false;
                    }
                }
        }
    }
}

void players::leftClickEvent(block this_block, connection& connection, unsigned short tick_length) {
    if(custom_block_events[(int)this_block.getType()].onLeftClick)
        custom_block_events[(int)this_block.getType()].onLeftClick(&this_block, getPlayerByConnection(&connection));
    else {
        this_block.setBreakProgress(this_block.getBreakProgress() + tick_length);
        if(this_block.getBreakProgress() >= this_block.getUniqueBlock().break_time)
            breakBlock(&this_block);
    }
}

void players::rightClickEvent(block this_block, player* peer) {
    if(custom_block_events[(int)this_block.getType()].onRightClick)
        custom_block_events[(int)this_block.getType()].onRightClick(&this_block, peer);
}

void players::saveTo(std::string path) {
    std::filesystem::create_directory(path);
    for(player* player : all_players) {
        std::ofstream data_file(path + player->name, std::ios::binary);
        for(auto& i : player->player_inventory.inventory_arr) {
            data_file << (char)i.getId();
            unsigned short stack = i.getStack();
            data_file.write((char*)&stack, sizeof(stack));
        }
        data_file.write((char*)&player->x, sizeof(player->x));
        data_file.write((char*)&player->y, sizeof(player->y));
        data_file.close();
    }
}

void players::loadFrom(std::string path) {
    for (const auto& entry : std::filesystem::directory_iterator(path)) {
        std::string player_name = entry.path().string();
        player_name = player_name.substr(player_name.find_last_of('/') + 1, player_name.size() - 1);
        player* player = getPlayerByName(player_name);

        std::ifstream data_file(entry.path(), std::ios::binary);
        for(auto & i : player->player_inventory.inventory_arr) {
            char c;
            data_file >> c;
            i.setId((itemType)c);

            unsigned short stack;
            data_file.read((char*)&stack, sizeof(stack));
            i.setStack(stack);
        }

        data_file.read((char*)&player->x, sizeof(player->x));
        data_file.read((char*)&player->y, sizeof(player->y));
    }
}

void players::breakBlock(block* this_block) {
    if(this_block->getUniqueBlock().drop != itemType::NOTHING)
        parent_items->spawnItem(this_block->getUniqueBlock().drop, this_block->getX() * BLOCK_WIDTH, this_block->getY() * BLOCK_WIDTH);
    blockType this_type = this_block->getType();
    this_block->setType(blockType::AIR);
    this_block->setBreakProgress(0);
    if(custom_block_events[(int)this_type].onBreak)
        custom_block_events[(int)this_type].onBreak(parent_blocks, this, this_block);
}
