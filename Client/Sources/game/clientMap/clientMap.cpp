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

void map::onEvent(ClientPacketEvent &event) {
    switch(event.packet.getType()) {
        case PacketType::ITEM_CREATION: {
            auto type = (ItemType)event.packet.get<char>();
            auto id = event.packet.get<unsigned short>();
            int y = event.packet.get<int>(), x = event.packet.get<int>();
            items.emplace_back(item(type, x, y, id));
            break;
        }
        case PacketType::ITEM_DELETION: {
            auto id = event.packet.get<unsigned short>();
            for(auto i = items.begin(); i != items.end(); i++)
                if(i->getId() == id) {
                    items.erase(i);
                    break;
                }
            break;
        }
        case PacketType::ITEM_MOVEMENT: {
            item* item = getItemById(event.packet.get<unsigned short>());
            item->y = event.packet.get<int>();
            item->x = event.packet.get<int>();
            break;
        }
        case PacketType::BLOCK_CHANGE: {
            auto type = (BlockType)event.packet.get<unsigned char>();
            auto light_level = event.packet.get<unsigned char>();
            auto liquid_level = event.packet.get<unsigned char>();
            auto liquid_type = (LiquidType)event.packet.get<unsigned char>();
            auto y = event.packet.get<unsigned short>(), x = event.packet.get<unsigned short>();
            block curr_block = getBlock(x, y);
            curr_block.setType(type, liquid_type);
            curr_block.setLiquidLevel(liquid_level);
            curr_block.setLightLevel(light_level);
            break;
        }
        case PacketType::CHUNK: {
            chunks_pending--;
            auto x = event.packet.get<unsigned short>(), y = event.packet.get<unsigned short>();
            
            for(unsigned short y_ = 0; y_ < 16; y_++)
                for(unsigned short x_ = 0; x_ < 16; x_++) {
                    auto light_level = event.packet.get<unsigned char>();
                    auto liquid_level = event.packet.get<unsigned char>();
                    auto liquid_type = (LiquidType)event.packet.get<unsigned char>();
                    auto type = (BlockType)event.packet.get<unsigned char>();
                    block block = getBlock((x << 4) + x_, (y << 4) + y_);
                    block.setType(type, liquid_type);
                    block.setLightLevel(light_level);
                    block.setLiquidLevel(liquid_level);
                    block.update();
                }
            
            getChunk(x, y).setState(map::chunkState::loaded);
            break;
        }
        case PacketType::BLOCK_PROGRESS_CHANGE: {
            auto stage = event.packet.get<unsigned char>();
            auto x = event.packet.get<unsigned short>(), y = event.packet.get<unsigned short>();
            getBlock(x, y).setBreakStage(stage);
            break;
        }
        case PacketType::KICK: {
            kick_message = event.packet.get<std::string>();
            kicked = true;
        }
        default:;
    }
}

void map::init() {
    background_image.loadFromFile("texturePack/misc/background.png");
}

void map::render() {
    float scale = (float)gfx::getWindowHeight() / background_image.getTextureHeight();
    int position_x = -(view_x / 5) % int(background_image.getTextureWidth() * scale);
    for(int i = 0; i < gfx::getWindowWidth() / (background_image.getTextureWidth() * scale) + 2; i++)
        background_image.render(scale, position_x + i * background_image.getTextureWidth() * scale, 0);
    renderBlocks();
    renderItems();
    if(kicked) {
        choiceScreen(kick_message, {"Close"}).run();
        gfx::returnFromScene();
    }
}

map::~map() {
    delete[] chunks;
    delete[] blocks;
}
