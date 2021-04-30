//
//  packets.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/04/2021.
//

#include "map.hpp"

void map::onPacket(packets::packet packet) {
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
            //removeNaturalLight(x);
            getBlock(x, y).setType(type);
            //setNaturalLight(x);
            //getBlock(x, y).lightUpdate();
            break;
        }
        case packets::CHUNK: {
            unsigned short x = packet.getUShort(), y = packet.getUShort();
            
            for(unsigned short y_ = 0; y_ < 16; y_++)
                for(unsigned short x_ = 0; x_ < 16; x_++) {
                    unsigned char light_level = packet.getUChar();
                    map::blockType type = (map::blockType)packet.getChar();
                    getBlock((x << 4) + x_, (y << 4) + y_).setType(type);
                    getBlock((x << 4) + x_, (y << 4) + y_).setLightLevel(light_level);
                }
            
            for(unsigned short y_ = 0; y_ < 18; y_++)
                for(unsigned short x_ = 0; x_ < 18; x_++)
                    if((x << 4) + x_ - 1 >= 0 && (x << 4) + x_ - 1 < getWorldWidth() && (y << 4) + y_ - 1 >= 0 && (y << 4) + y_ - 1 < getWorldHeight())
                        getBlock((x << 4) + x_ - 1, (y << 4) + y_ - 1).scheduleTextureUpdate();
            
            getChunk(x, y).setState(map::chunkState::loaded);
            break;
        }
        case packets::BLOCK_PROGRESS_CHANGE: {
            unsigned char stage = packet.getUChar();
            unsigned short x = packet.getUShort(), y = packet.getUShort();
            getBlock(x, y).setBreakStage(stage);
            break;
        }
        default:;
    }
}
