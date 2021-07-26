//
//  map.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/04/2021.
//

#include "clientMap.hpp"
#include "choiceScreen.hpp"
#include "fileManager.hpp"

void map::createWorld(unsigned short map_width, unsigned short map_height) {
    chunks = new chunkData[map_width * map_height];
    blocks = new blockData[(map_width << 4) * (map_height << 4)];
    width = map_width << 4;
    height = map_height << 4;
}

void map::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case PacketType::ITEM_CREATION: {
            int x, y;
            unsigned short id;
            unsigned char type_char;
            event.packet >> x >> y >> id >> type_char;
            ItemType type = (ItemType)type_char;
            
            items.emplace_back(item(type, x, y, id));
            break;
        }
        case PacketType::ITEM_DELETION: {
            unsigned short id;
            event.packet >> id;
            for(auto i = items.begin(); i != items.end(); i++)
                if(i->getId() == id) {
                    items.erase(i);
                    break;
                }
            break;
        }
        case PacketType::ITEM_MOVEMENT: {
            unsigned short id;
            int x, y;
            event.packet >> x >> y >> id;
            
            item* item = getItemById(id);
            item->x = x;
            item->y = y;
            break;
        }
        case PacketType::BLOCK_CHANGE: {
            unsigned short x, y;
            unsigned char block_type;
            event.packet >> x >> y >> block_type;
            
            block curr_block = getBlock(x, y);
            curr_block.setType((BlockType)block_type, curr_block.getLiquidType());
            break;
        }
        case PacketType::LIGHT_CHANGE: {
            unsigned short x, y;
            unsigned char light_level;
            event.packet >> x >> y >> light_level;
            
            getBlock(x, y).setLightLevel(light_level);
            break;
        }
        case PacketType::LIQUID_CHANGE: {
            unsigned short x, y;
            unsigned char liquid_type, liquid_level;
            event.packet >> x >> y >> liquid_type >> liquid_level;
            
            block curr_block = getBlock(x, y);
            curr_block.setType(curr_block.getType(), (LiquidType)liquid_type);
            curr_block.setLiquidLevel(liquid_level);
            break;
        }
        case PacketType::CHUNK: {
            unsigned short x, y;
            event.packet >> x >> y;
            
            for(unsigned short chunk_x = 0; chunk_x < 16; chunk_x++)
                for(unsigned short chunk_y = 0; chunk_y < 16; chunk_y++) {
                    unsigned char block_type, liquid_type, liquid_level, light_level;
                    event.packet >> block_type >> liquid_type >> liquid_level >> light_level;
                    
                    block block = getBlock((x << 4) + chunk_x, (y << 4) + chunk_y);
                    block.setType((BlockType)block_type, (LiquidType)liquid_type);
                    block.setLightLevel(light_level);
                    block.setLiquidLevel(liquid_level);
                    block.update();
                }
            
            getChunk(x, y).setState(map::chunkState::loaded);
            getChunk(x, y).createTexture();
            break;
        }
        case PacketType::BLOCK_PROGRESS_CHANGE: {
            unsigned char stage;
            unsigned short x, y;
            event.packet >> x >> y >> stage;
            getBlock(x, y).setBreakStage(stage);
            break;
        }
        case PacketType::KICK: {
            event.packet >> kick_message;
            kicked = true;
        }
        default:;
    }
}

void map::init() {
    background_image.loadFromFile("resourcePack/misc/background.png");
}

void map::render() {
    float scale = (float)gfx::getWindowHeight() / background_image.getTextureHeight();
    int position_x = -(view_x / 5) % int(background_image.getTextureWidth() * scale);
    for(int i = 0; i < gfx::getWindowWidth() / (background_image.getTextureWidth() * scale) + 2; i++)
        background_image.render(scale, position_x + i * background_image.getTextureWidth() * scale, 0);
    renderBlocksBack();
    renderItems();
    if(kicked) {
        choiceScreen(kick_message, {"Close"}).run();
        gfx::returnFromScene();
    }
}

void mapFront::render() {
    world_map->renderBlocksFront();
}

map::~map() {
    delete[] chunks;
    delete[] blocks;
}
