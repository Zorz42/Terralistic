//
//  blockEngine.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 12/01/2021.
//

#include "core.hpp"

#include <chrono>
#include "main.hpp"
#include "blockEngine.hpp"
#include "print.hpp"
#include "playerHandler.hpp"
#include "clickEvents.hpp"

EVENT_LISTENER(blockEngine::block_change)
        packets::packet packet(packets::BLOCK_CHANGE);
        packet << data.x << data.y << (unsigned char) data.type;
        networking::sendToEveryone(packet);
EVENT_LISTENER_END

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
    if(clickEvents::click_events[block->block_id].rightClickEvent)
        clickEvents::click_events[block->block_id].rightClickEvent(block, playerHandler::getPlayerByConnection(&connection));
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
    if(clickEvents::click_events[block->block_id].leftClickEvent)
        clickEvents::click_events[block->block_id].leftClickEvent(block, playerHandler::getPlayerByConnection(&connection));
    else {
        char prev = block->break_progress;
        block->setBreakProgress(block->break_progress_ms + main_::frame_length);
        if(block->break_progress != prev) {
            packets::packet packet(packets::BLOCK_BREAK_PROGRESS_CHANGE);
            packet << y << x << block->break_progress_ms;
            networking::sendToEveryone(packet);
        }
        if(block->break_progress_ms >= block->getUniqueBlock().break_time)
            blockEngine::getBlock(x, y).break_block();
    }
}

void networking::updatePlayersBreaking() {
    for(playerHandler::player& player : playerHandler::players)
        if(player.breaking)
            leftClickEvent(player.breaking_x, player.breaking_y, *player.conn);
}
