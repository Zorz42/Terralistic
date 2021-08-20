#include <cassert>
#include "clientBlocks.hpp"
#include "fileManager.hpp"

void ClientBlocks::createWorld(unsigned short map_width, unsigned short map_height) {
    assert(map_width % 16 == 0 && map_height % 16 == 0);
    width = map_width;
    height = map_height;
    chunks = new ClientMapChunk[(width >> 4) * (height >> 4)];
    blocks = new ClientMapBlock[width * height];
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
        case PacketType::CHUNK: {
            unsigned short x, y;
            event.packet >> x >> y;
            
            for(unsigned short chunk_x = 0; chunk_x < 16; chunk_x++)
                for(unsigned short chunk_y = 0; chunk_y < 16; chunk_y++) {
                    unsigned char block_type, liquid_type, liquid_level, light_level;
                    event.packet >> block_type >> liquid_type >> liquid_level >> light_level;
                    
                    ClientBlock block = getBlock((x << 4) + chunk_x, (y << 4) + chunk_y);
                    block.setType((BlockType)block_type, (LiquidType)liquid_type);
                    block.setLightLevel(light_level);
                    block.setLiquidLevel(liquid_level);
                    block.scheduleTextureUpdate();
                    block.scheduleTextureUpdateForNeighbors();
                }
            
            getChunk(x, y).setState(ChunkState::loaded);
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
        default:;
    }
}

ClientBlocks::~ClientBlocks() {
    delete[] chunks;
    delete[] blocks;
}
