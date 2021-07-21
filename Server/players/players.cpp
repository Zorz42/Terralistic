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

static bool isBlockTree(Block block) {
    return block.refersToABlock() && (block.getBlockType() == BlockType::WOOD || block.getBlockType() == BlockType::LEAVES);
}

static bool isBlockWood(Block block) {
    return block.refersToABlock() && block.getBlockType() == BlockType::WOOD;
}

static bool isBlockLeaves(Block block) {
    return block.refersToABlock() && block.getBlockType() == BlockType::LEAVES;
}

players::players(Blocks* parent_blocks_, Items* parent_items_) : parent_blocks(parent_blocks_), parent_items(parent_items_) {
    custom_block_events[(int)BlockType::WOOD].onUpdate = [](Blocks* server_blocks, Block* this_block) {
        Block upper, lower, left, right;
        if(this_block->getY() != 0)
            upper = server_blocks->getBlock(this_block->getX(), this_block->getY() - 1);
        if(this_block->getY() != server_blocks->getHeight() - 1)
            lower = server_blocks->getBlock(this_block->getX(), this_block->getY() + 1);
        if(this_block->getX() != 0)
            left = server_blocks->getBlock(this_block->getX() - 1, this_block->getY());
        if(this_block->getX() != server_blocks->getWidth() - 1)
            right = server_blocks->getBlock(this_block->getX() + 1, this_block->getY());
        
        if(
           (!isBlockTree(lower) && !isBlockTree(left) && !isBlockTree(right)) ||
           (isBlockWood(upper) && isBlockWood(right) && !isBlockTree(left) && !isBlockTree(lower)) ||
           (isBlockWood(upper) && isBlockWood(left) && !isBlockTree(right) && !isBlockTree(lower)) ||
           (isBlockLeaves(left) && !isBlockTree(right) && !isBlockTree(upper) && !isBlockTree(lower)) ||
           (isBlockLeaves(right) && !isBlockTree(left) && !isBlockTree(upper) && !isBlockTree(lower)) ||
           (!isBlockTree(lower) && isBlockLeaves(left) && isBlockLeaves(right) && isBlockLeaves(upper))
           )
            this_block->breakBlock();
    };

    custom_block_events[(int)BlockType::LEAVES].onUpdate = custom_block_events[(int)BlockType::WOOD].onUpdate;

    custom_block_events[(int)BlockType::GRASS_BLOCK].onLeftClick = [](Block* this_block, Player* peer) {
        this_block->setType(BlockType::DIRT);
    };

    custom_block_events[(int)BlockType::AIR].onRightClick = [](Block* this_block, Player* peer) {
        BlockType type = peer->inventory.getSelectedSlot()->getUniqueItem().places;
        if(type != BlockType::AIR && peer->inventory.inventory_arr[peer->inventory.selected_slot].decreaseStack(1)) {
            this_block->setType(type);
            this_block->update();
        }
    };

    custom_block_events[(int)BlockType::SNOWY_GRASS_BLOCK].onLeftClick = custom_block_events[(int)BlockType::GRASS_BLOCK].onLeftClick;
}

players::~players() {
    for(Player* i : all_players)
        delete i;
}

Player* players::getPlayerByName(const std::string& name) {
    for(Player* player : all_players)
        if(player->name == name)
            return player;
    
    all_players.emplace_back(new Player);
    Player* curr_player = all_players.back();
    curr_player->y = parent_blocks->getSpawnY() - BLOCK_WIDTH * 2;
    curr_player->x = parent_blocks->getSpawnX();
    curr_player->name = name;
    return curr_player;
}

void players::updatePlayersBreaking(unsigned short tick_length) {
    for(Player* player : online_players)
        if(player->breaking)
            leftClickEvent(parent_blocks->getBlock(player->breaking_x, player->breaking_y), player, tick_length);
}

void players::lookForItems() {
    for(const Item& i : parent_items->getItems())
        for(Player* player : online_players)
            if(abs(i.getX() / 100 + BLOCK_WIDTH / 2  - player->x - 14) < 50 && abs(i.getY() / 100 + BLOCK_WIDTH / 2 - player->y - 25) < 50)
                if(player->inventory.addItem(i.getType(), 1) != -1)
                    parent_items->removeItem(i);
}

void players::updateBlocks() {
    for(Player* player : online_players) {
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
        bool finished = false;
        while(!finished) {
            finished = true;
            for(unsigned short y = start_y; y < end_y; y++)
                for(unsigned short x = start_x; x < end_x; x++) {
                    Block curr_block = parent_blocks->getBlock(x, y);
                    if(curr_block.hasScheduledLightUpdate()) {
                        curr_block.lightUpdate();
                        finished = false;
                    }
                    if(curr_block.getLiquidType() != LiquidType::EMPTY && curr_block.canUpdateLiquid()) {
                        curr_block.liquidUpdate();
                        finished = false;
                    }
                }
        }
        for(unsigned short y = start_y; y < end_y; y++)
            for(unsigned short x = start_x; x < end_x; x++) {
                Block curr_block = parent_blocks->getBlock(x, y);
                if(curr_block.hasLightChanged()) {
                    curr_block.markLightUnchanged();
                    sf::Packet packet;
                    packet << PacketType::LIGHT_CHANGE << x << y << (unsigned char)curr_block.getLightLevel();
                    sendToEveryone(packet);
                }
            }
    }
}

void players::leftClickEvent(Block this_block, Player* peer, unsigned short tick_length) {
    if(custom_block_events[(int)this_block.getBlockType()].onLeftClick)
        custom_block_events[(int)this_block.getBlockType()].onLeftClick(&this_block, peer);
    else if(this_block.getUniqueBlock().break_time != UNBREAKABLE) {
        this_block.setBreakProgress(this_block.getBreakProgress() + tick_length);
        if(this_block.getBreakProgress() >= this_block.getUniqueBlock().break_time)
            this_block.breakBlock();
    }
}

void players::rightClickEvent(Block this_block, Player* peer) {
    if(custom_block_events[(int)this_block.getBlockType()].onRightClick)
        custom_block_events[(int)this_block.getBlockType()].onRightClick(&this_block, peer);
}

void players::saveTo(std::string path) {
    std::filesystem::create_directory(path);
    for(Player* player : all_players) {
        std::ofstream data_file(path + player->name, std::ios::binary);
        for(auto& i : player->inventory.inventory_arr) {
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
        Player* player = getPlayerByName(player_name);

        std::ifstream data_file(entry.path(), std::ios::binary);
        for(auto & i : player->inventory.inventory_arr) {
            char c;
            data_file >> c;
            i.setIdWithoutProcessing((ItemType)c);

            unsigned short stack;
            data_file.read((char*)&stack, sizeof(stack));
            i.setStackWithoutProcessing(stack);
        }

        data_file.read((char*)&player->x, sizeof(player->x));
        data_file.read((char*)&player->y, sizeof(player->y));
    }
}

void players::onEvent(ServerBlockUpdateEvent& event) {
    if(custom_block_events[(int)event.block.getBlockType()].onUpdate)
        custom_block_events[(int)event.block.getBlockType()].onUpdate(parent_blocks, &event.block);
}
