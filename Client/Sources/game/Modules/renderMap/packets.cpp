//
//  packets.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/04/2021.
//

#include "renderMap.hpp"

void renderMap::onPacket(packets::packet packet) {
    switch(packet.type) {
        case packets::ITEM_CREATION: {
            auto type = (map::itemType)packet.getChar();
            unsigned short id = packet.getUShort();
            int y = packet.getInt(), x = packet.getInt();
            spawnItem(type, x, y, id);
            break;
        }
        case packets::ITEM_DELETION: {
            unsigned short id = packet.getUShort();
            for(auto i = items.begin(); i != items.end(); i++)
                if(i->getId() == id) {
                    items.erase(i);
                    break;
                }
            break;
        }
        case packets::ITEM_MOVEMENT: {
            map::item* item = getItemById(packet.getUShort());
            item->y = packet.getInt();
            item->x = packet.getInt();
            break;
        }
        case packets::BLOCK_CHANGE: {
            auto type = (map::blockType)packet.getUChar();
            unsigned short y = packet.getUShort(), x = packet.getUShort();
            removeNaturalLight(x);
            getBlock(x, y).setType(type);
            setNaturalLight(x);
            getBlock(x, y).lightUpdate();
            break;
        }
        case packets::CHUNK: {
            unsigned short x = packet.getUShort(), y = packet.getUShort();
            for(unsigned short x_ = x << 4; x_ < (x << 4) + 16; x_++)
                removeNaturalLight(x_);
            for(unsigned short y_ = 0; y_ < 16; y_++)
                for(unsigned short x_ = 0; x_ < 16; x_++) {
                    map::blockType type = (map::blockType)packet.getChar();
                    getBlock((x << 4) + x_, (y << 4) + y_).setType(type);
                }
            for(unsigned short x_ = x << 4; x_ < (x << 4) + 16; x_++)
                setNaturalLight(x_);
            getChunkState(x, y) = map::chunkState::loaded;
            break;
        }
        case packets::BLOCK_PROGRESS_CHANGE: {
            unsigned short progress = packet.getUShort(), x = packet.getUShort(), y = packet.getUShort();
            getBlock(x, y).setBreakProgress(progress);
            break;
        }
        default:;
    }
}
