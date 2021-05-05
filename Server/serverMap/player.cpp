//
//  player.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 05/05/2021.
//

#include "serverMap.hpp"
#include "clickEvents.hpp"

map::player* map::getPlayerByConnection(connection* conn) {
    for(player& player : players)
        if(player.conn == conn)
            return &player;
    return nullptr;
}

void map::lookForItems(map& world_map) {
    for(unsigned long i = 0; i < world_map.items.size(); i++) {
        for(player& player : players)
            if(abs(world_map.items[i].x / 100 + BLOCK_WIDTH / 2  - player.x - 14) < 50 && abs(world_map.items[i].y / 100 + BLOCK_WIDTH / 2 - player.y - 25) < 50) {
                char result = player.inventory.addItem(world_map.items[i].getItemId(), 1);
                if(result != -1) {
                    packets::packet item_receive_packet(packets::INVENTORY_CHANGE);
                    item_receive_packet << (unsigned char)player.inventory.inventory[result].item_id << (unsigned short)player.inventory.inventory[result].getStack() << result;
                    player.conn->sendPacket(item_receive_packet);
                    world_map.items[i].destroy(world_map);
                    world_map.items.erase(world_map.items.begin() + i);
                }
            }
    }
}

void map::updateLight() {
    for(map::player& player : players) {
        for(unsigned short x = player.x / 16 - player.sight_width / 2 - 20; x < player.x / 16 + player.sight_width / 2 + 20; x++)
            for(unsigned short y = player.y / 16 - player.sight_height / 2 - 20; y < player.y / 16 + player.sight_height / 2 + 20; y++)
                if(getBlock(x, y).hasScheduledLightUpdate())
                    getBlock(x, y).lightUpdate();
    }
}

int map::getSpawnX() {
    return getWorldWidth() / 2 * BLOCK_WIDTH;
}

int map::getSpawnY() {
    int result = 0;
    for(unsigned short y = 0; y < getWorldHeight(); y++) {
        if(!getBlock(getWorldWidth() / 2 - 1, y).isTransparent() || !getBlock(getWorldWidth() / 2, y).isTransparent())
            break;
        result += BLOCK_WIDTH;
    }
    return result;
}

void leftClickEvent(unsigned short x, unsigned short y, connection& connection, map& world_map, unsigned short tick_length) {
    map::block block = world_map.getBlock(x, y);
    if(clickEvents::click_events[(int)block.getType()].leftClickEvent)
        clickEvents::click_events[(int)block.getType()].leftClickEvent(&block, world_map.getPlayerByConnection(&connection));
    else {
        block.setBreakProgress(block.getBreakProgress() + tick_length);
        if(block.getBreakProgress() >= block.getBreakTime())
            block.breakBlock();
    }
}

void map::updatePlayersBreaking(unsigned short tick_length) {
    for(player& player : players)
        if(player.breaking)
            leftClickEvent(player.breaking_x, player.breaking_y, *player.conn, *this, tick_length);
}
