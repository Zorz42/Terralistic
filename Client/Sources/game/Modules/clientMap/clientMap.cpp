//
//  map.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/04/2021.
//

#include "clientMap.hpp"
#include "assert.hpp"
#include "choiceScreen.hpp"
#include "fileManager.hpp"

void map::createWorld(unsigned short map_width, unsigned short map_height) {
    chunks = new chunkData[map_width * map_height];
    blocks = new blockData[(map_width << 4) * (map_height << 4)];
    width = map_width << 4;
    height = map_height << 4;
    
    for(unsigned short x = 0; x < (getWorldWidth() >> 4); x++)
        for(unsigned short y = 0; y < (getWorldHeight() >> 4); y++)
            getChunk(x, y).createTexture();
}

void map::onPacket(packets::packet &packet) {
    switch(packet.type) {
        case packets::ITEM_CREATION: {
            auto type = (map::itemType)packet.get<char>();
            unsigned short id = packet.get<unsigned short>();
            int y = packet.get<int>(), x = packet.get<int>();
            items.emplace_back(item(type, x, y, id));
            break;
        }
        case packets::ITEM_DELETION: {
            unsigned short id = packet.get<unsigned short>();
            for(auto i = items.begin(); i != items.end(); i++)
                if(i->getId() == id) {
                    items.erase(i);
                    break;
                }
            break;
        }
        case packets::ITEM_MOVEMENT: {
            item* item = getItemById(packet.get<unsigned short>());
            item->y = packet.get<int>();
            item->x = packet.get<int>();
            break;
        }
        case packets::BLOCK_CHANGE: {
            blockType type = (blockType)packet.get<unsigned char>();
            unsigned char light_level = packet.get<unsigned char>();
            unsigned char liquid_level = packet.get<unsigned char>();
            liquidType liquid_type = (liquidType)packet.get<unsigned char>();
            unsigned short y = packet.get<unsigned short>(), x = packet.get<unsigned short>();
            block curr_block = getBlock(x, y);
            curr_block.setType(type, liquid_type);
            curr_block.setLiquidLevel(liquid_level);
            curr_block.setLightLevel(light_level);
            break;
        }
        case packets::CHUNK: {
            chunks_pending--;
            unsigned short x = packet.get<unsigned short>(), y = packet.get<unsigned short>();
            
            for(unsigned short y_ = 0; y_ < 16; y_++)
                for(unsigned short x_ = 0; x_ < 16; x_++) {
                    unsigned char light_level = packet.get<unsigned char>();
                    unsigned char liquid_level = packet.get<unsigned char>();
                    liquidType liquid_type = (liquidType)packet.get<unsigned char>();
                    blockType type = (blockType)packet.get<unsigned char>();
                    block block = getBlock((x << 4) + x_, (y << 4) + y_);
                    block.setType(type, liquid_type);
                    block.setLightLevel(light_level);
                    block.setLiquidLevel(liquid_level);
                    block.update();
                }
            
            getChunk(x, y).setState(map::chunkState::loaded);
            break;
        }
        case packets::BLOCK_PROGRESS_CHANGE: {
            unsigned char stage = packet.get<unsigned char>();
            unsigned short x = packet.get<unsigned short>(), y = packet.get<unsigned short>();
            getBlock(x, y).setBreakStage(stage);
            break;
        }
        case packets::KICK: {
            kick_message = packet.get<std::string>();
            kicked = true;
        }
        default:;
    }
}

void map::init() {
    background_image.setTexture(gfx::loadImageFile("texturePack/misc/background.png"));
}

void map::render() {
    background_image.scale = (float)gfx::getWindowHeight() / background_image.getTextureHeight();
    int position_x = -(view_x / 5) % int(background_image.getTextureWidth() * background_image.scale);
    for(int i = 0; i < gfx::getWindowWidth() / (background_image.getTextureWidth() * background_image.scale) + 2; i++)
        gfx::render(background_image, position_x + i * background_image.getTextureWidth() * background_image.scale, 0);
    renderBlocks();
    renderItems();
    if(kicked) {
        gfx::runScene(new choiceScreen(kick_message, {"Close"}));
        gfx::returnFromScene();
    }
}

map::~map() {
    if(chunks)
        delete[] chunks;
    if(blocks)
        delete[] blocks;
}
