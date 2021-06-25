//
//  players.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 22/06/2021.
//

#include "players.hpp"
#include "blocks.hpp"

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
    //for(player* player : online_players)
        //if(player->breaking)
            //parent_blocks->getBlock(player->breaking_x, player->breaking_y).leftClickEvent(*player->conn, tick_length);
}

void players::lookForItems() {
    for(unsigned long i = 0; i < parent_items->item_arr.size(); i++) {
        for(player* player : online_players)
            if(abs(parent_items->item_arr[i].x / 100 + BLOCK_WIDTH / 2  - player->x - 14) < 50 && abs(parent_items->item_arr[i].y / 100 + BLOCK_WIDTH / 2 - player->y - 25) < 50) {
                char result = player->player_inventory.addItem(parent_items->item_arr[i].getItemId(), 1);
                if(result != -1) {
                    parent_items->item_arr[i].destroy();
                    parent_items->item_arr.erase(parent_items->item_arr.begin() + i);
                }
            }
    }
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
            if(end_y > parent_blocks->height)
                end_y = parent_blocks->height;
            if(end_x > parent_blocks->width)
                end_x = parent_blocks->width;
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
