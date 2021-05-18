//
//  player.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 05/05/2021.
//

#include "serverMap.hpp"
#include "clickEvents.hpp"

serverMap::player* serverMap::getPlayerById(unsigned short id) {
    for(player* player : online_players)
        if(player->id == id)
            return player;
    return nullptr;
}

serverMap::player* serverMap::getPlayerByConnection(connection* conn) {
    for(player* player : online_players)
        if(player->conn == conn)
            return player;
    return nullptr;
}

void serverMap::lookForItems(serverMap& world_serverMap) {
    for(unsigned long i = 0; i < world_serverMap.items.size(); i++) {
        for(player* player : online_players)
            if(abs(world_serverMap.items[i].x / 100 + BLOCK_WIDTH / 2  - player->x - 14) < 50 && abs(world_serverMap.items[i].y / 100 + BLOCK_WIDTH / 2 - player->y - 25) < 50) {
                char result = player->inventory.addItem(world_serverMap.items[i].getItemId(), 1);
                if(result != -1) {
                    world_serverMap.items[i].destroy(world_serverMap);
                    world_serverMap.items.erase(world_serverMap.items.begin() + i);
                }
            }
    }
}

void serverMap::updateLight() {
    for(player* player : online_players) {
        for(unsigned short x = player->x / 16 - player->sight_width / 2 - 20; x < player->x / 16 + player->sight_width / 2 + 20; x++)
            for(unsigned short y = player->y / 16 - player->sight_height / 2 - 20; y < player->y / 16 + player->sight_height / 2 + 20; y++)
                if(getBlock(x, y).hasScheduledLightUpdate())
                    getBlock(x, y).lightUpdate();
    }
}

int serverMap::getSpawnX() {
    return getWorldWidth() / 2 * BLOCK_WIDTH;
}

int serverMap::getSpawnY() {
    int result = 0;
    for(unsigned short y = 0; y < getWorldHeight(); y++) {
        if(!getBlock(getWorldWidth() / 2 - 1, y).isTransparent() || !getBlock(getWorldWidth() / 2, y).isTransparent())
            break;
        result += BLOCK_WIDTH;
    }
    return result;
}

void leftClickEvent(unsigned short x, unsigned short y, connection& connection, serverMap& world_serverMap, unsigned short tick_length) {
    serverMap::block block = world_serverMap.getBlock(x, y);
    if(clickEvents::click_events[(int)block.getType()].leftClickEvent)
        clickEvents::click_events[(int)block.getType()].leftClickEvent(&block, world_serverMap.getPlayerByConnection(&connection));
    else {
        block.setBreakProgress(block.getBreakProgress() + tick_length);
        if(block.getBreakProgress() >= block.getBreakTime())
            block.breakBlock();
    }
}

void serverMap::updatePlayersBreaking(unsigned short tick_length) {
    for(player* player : online_players)
        if(player->breaking)
            leftClickEvent(player->breaking_x, player->breaking_y, *player->conn, *this, tick_length);
}

serverMap::player* serverMap::getPlayerByName(const std::string& name) {
    static unsigned int curr_id = 0;
    for(player& player : all_players)
        if(player.name == name)
            return &player;
    all_players.emplace_back(player(curr_id++, this));
    player& curr_player = all_players.back();
    curr_player.y = getSpawnY() - BLOCK_WIDTH * 2;
    curr_player.x = getSpawnX();
    curr_player.name = name;
    return &curr_player;
}
