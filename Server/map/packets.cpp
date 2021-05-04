//
//  packets.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 04/05/2021.
//

#include <chrono>
#include "main.hpp"
#include "print.hpp"
#include "playerHandler.hpp"
#include "clickEvents.hpp"
#include "packets.hpp"
#include "networkingModule.hpp"
#include "map.hpp"

void map::onPacket(packets::packet& packet, connection& conn) {
    player* player = getPlayerByConnection(&conn);
    switch (packet.type) {
        case packets::STARTED_BREAKING: {
            unsigned short y = packet.getUShort(), x = packet.getUShort();
            player->breaking_x = x;
            player->breaking_y = y;
            player->breaking = true;
            break;
        }
            
        case packets::STOPPED_BREAKING: {
            player->breaking = false;
            break;
        }
            
        case packets::RIGHT_CLICK: {
            unsigned short y = packet.getUShort(), x = packet.getUShort();
            map::block block = getBlock(x, y);
            if(clickEvents::click_events[(int)block.getType()].rightClickEvent)
                clickEvents::click_events[(int)block.getType()].rightClickEvent(&block, player);
            break;
        }
            
        case packets::CHUNK: {
            packets::packet chunk_packet(packets::CHUNK);
            unsigned short x = packet.getUShort(), y = packet.getUShort();
            for(int i = 0; i < 16 * 16; i++) {
                map::block block = getBlock((x << 4) + 15 - i % 16, (y << 4) + 15 - i / 16);
                chunk_packet << (char)block.getType() << (unsigned char)block.getLightLevel();
            }
            chunk_packet << y << x;
            conn.sendPacket(chunk_packet);
            break;
        }
            
        case packets::VIEW_SIZE_CHANGE: {
            unsigned short width = packet.getUShort(), height = packet.getUShort();
            player->sight_width = width;
            player->sight_height = height;
            break;
        }
            
        default:;
    }
}
