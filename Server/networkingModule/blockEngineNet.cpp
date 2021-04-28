//
//  blockEngine.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 12/01/2021.
//

#include <chrono>
#include "main.hpp"
#include "print.hpp"
#include "playerHandler.hpp"
#include "clickEvents.hpp"
#include "packets.hpp"
#include "networkingModule.hpp"

/*EVENT_LISTENER(blockEngine::block_change)
        packets::packet packet(packets::BLOCK_CHANGE);
        packet << data.x << data.y << (unsigned char) data.type;
        networking::sendToEveryone(packet);
EVENT_LISTENER_END*/

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
    map::block block = playerHandler::world_map->getBlock(x, y);
    if(clickEvents::click_events[(int)block.getType()].rightClickEvent)
        clickEvents::click_events[(int)block.getType()].rightClickEvent(&block, playerHandler::getPlayerByConnection(&connection));
PACKET_LISTENER_END

PACKET_LISTENER(packets::CHUNK)
    packets::packet chunk_packet(packets::CHUNK);
    unsigned short x = packet.getUShort(), y = packet.getUShort();
    for(int i = 0; i < 16 * 16; i++)
        chunk_packet << (char)playerHandler::world_map->getBlock((x << 4) + 15 - i % 16, (y << 4) + 15 - i / 16).getType();
    chunk_packet << y << x;
    connection.sendPacket(chunk_packet);
PACKET_LISTENER_END

void leftClickEvent(unsigned short x, unsigned short y, networking::connection& connection, map& world_map) {
    map::block block = world_map.getBlock(x, y);
    if(clickEvents::click_events[(int)block.getType()].leftClickEvent)
        clickEvents::click_events[(int)block.getType()].leftClickEvent(&block, playerHandler::getPlayerByConnection(&connection));
    else {
        char prev = block.getBreakStage();
        block.setBreakProgress(block.getBreakProgress() + main_::frame_length);
        if(block.getBreakStage() != prev) {
            packets::packet packet(packets::BLOCK_PROGRESS_CHANGE);
            packet << y << x << block.getBreakProgress();
            networking::sendToEveryone(packet);
        }
        if(block.getBreakProgress() >= block.getBreakTime())
            block.breakBlock();
    }
}

void networking::updatePlayersBreaking(map& world_map) {
    for(playerHandler::player& player : playerHandler::players)
        if(player.breaking)
            leftClickEvent(player.breaking_x, player.breaking_y, *player.conn, world_map);
}
