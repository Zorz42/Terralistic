//
//  players.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 22/06/2021.
//

#include "players.hpp"
#include "blocks.hpp"

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
    curr_player->y = getSpawnY() - BLOCK_WIDTH * 2;
    curr_player->x = getSpawnX();
    curr_player->name = name;
    return curr_player;
}

void players::updatePlayersBreaking(unsigned short tick_length) {
    for(player* player : online_players)
        if(player->breaking)
            getBlock(player->breaking_x, player->breaking_y).leftClickEvent(*player->conn, tick_length);
}

void players::lookForItems() {
    for(unsigned long i = 0; i < items.size(); i++) {
        for(player* player : online_players)
            if(abs(items[i].x / 100 + BLOCK_WIDTH / 2  - player->x - 14) < 50 && abs(items[i].y / 100 + BLOCK_WIDTH / 2 - player->y - 25) < 50) {
                char result = player->player_inventory.addItem(world_serverMap.items[i].getItemId(), 1);
                if(result != -1) {
                    items[i].destroy(world_serverMap);
                    items.erase(items.begin() + i);
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
            if(end_y > height)
                end_y = height;
            if(end_x > width)
                end_x = width;
            for(unsigned short x = start_x; x < end_x; x++)
                for(unsigned short y = end_y - 1; y >= start_y; y--) {
                    block curr_block = getBlock(x, y);
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
