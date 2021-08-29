#include <cassert>
#include "clientBlocks.hpp"
#include "fileManager.hpp"

void ClientBlocks::createWorld(unsigned short map_width, unsigned short map_height) {
    assert(map_width % 16 == 0 && map_height % 16 == 0);
    width = map_width;
    height = map_height;
    blocks = new ClientMapBlock[width * height];
    vertex_array.setPrimitiveType(sf::Quads);
}

void ClientBlocks::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case PacketType::BLOCK_CHANGE: {
            unsigned short x, y;
            unsigned char block_type;
            event.packet >> x >> y >> block_type;
            
            ClientBlock curr_block = getBlock(x, y);
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
            
            ClientBlock curr_block = getBlock(x, y);
            curr_block.setType(curr_block.getBlockType(), (LiquidType)liquid_type);
            curr_block.setLiquidLevel(liquid_level);
            break;
        }
        case PacketType::BLOCK_PROGRESS_CHANGE: {
            unsigned char stage;
            unsigned short x, y;
            event.packet >> x >> y >> stage;
            getBlock(x, y).setBreakStage(stage);
            break;
        }
        default:;
    }
}

ClientBlocks::~ClientBlocks() {
    delete[] blocks;
}
