#include "clientBlocks.hpp"
#include "platform_folders.h"

ClientBlocks::ClientBlocks(NetworkingManager* manager, ResourcePack* resource_pack) : networking_manager(manager), resource_pack(resource_pack) {
    stateFunctions[(int)BlockType::DIRT] = std::vector<void (ClientBlocks::*)(unsigned short, unsigned short)>{&ClientBlocks::updateOrientationLeft, &ClientBlocks::updateOrientationDown, &ClientBlocks::updateOrientationRight, &ClientBlocks::updateOrientationUp};
    stateFunctions[(int)BlockType::STONE_BLOCK] = std::vector<void (ClientBlocks::*)(unsigned short, unsigned short)>{&ClientBlocks::updateOrientationLeft, &ClientBlocks::updateOrientationDown, &ClientBlocks::updateOrientationRight, &ClientBlocks::updateOrientationUp};
    stateFunctions[(int)BlockType::GRASS_BLOCK] = std::vector<void (ClientBlocks::*)(unsigned short, unsigned short)>{&ClientBlocks::updateOrientationLeft, &ClientBlocks::updateOrientationDown, &ClientBlocks::updateOrientationRight, &ClientBlocks::updateOrientationUp};
    stateFunctions[(int)BlockType::WOOD] = std::vector<void (ClientBlocks::*)(unsigned short, unsigned short)>{&ClientBlocks::updateOrientationLeft, &ClientBlocks::updateOrientationDown, &ClientBlocks::updateOrientationRight, &ClientBlocks::updateOrientationUp};
    stateFunctions[(int)BlockType::LEAVES] = std::vector<void (ClientBlocks::*)(unsigned short, unsigned short)>{&ClientBlocks::updateOrientationLeft, &ClientBlocks::updateOrientationDown, &ClientBlocks::updateOrientationRight, &ClientBlocks::updateOrientationUp};
    stateFunctions[(int)BlockType::SAND] = std::vector<void (ClientBlocks::*)(unsigned short, unsigned short)>{&ClientBlocks::updateOrientationLeft, &ClientBlocks::updateOrientationDown, &ClientBlocks::updateOrientationRight, &ClientBlocks::updateOrientationUp};
    stateFunctions[(int)BlockType::SNOWY_GRASS_BLOCK] = std::vector<void (ClientBlocks::*)(unsigned short, unsigned short)>{&ClientBlocks::updateOrientationLeft, &ClientBlocks::updateOrientationDown, &ClientBlocks::updateOrientationRight, &ClientBlocks::updateOrientationUp};
    stateFunctions[(int)BlockType::SNOW_BLOCK] = std::vector<void (ClientBlocks::*)(unsigned short, unsigned short)>{&ClientBlocks::updateOrientationLeft, &ClientBlocks::updateOrientationDown, &ClientBlocks::updateOrientationRight, &ClientBlocks::updateOrientationUp};
    stateFunctions[(int)BlockType::ICE] = std::vector<void (ClientBlocks::*)(unsigned short, unsigned short)>{&ClientBlocks::updateOrientationLeft, &ClientBlocks::updateOrientationDown, &ClientBlocks::updateOrientationRight, &ClientBlocks::updateOrientationUp};

}
void ClientBlocks::create(unsigned short map_width, unsigned short map_height, const std::vector<char>& map_data) {
    width = map_width;
    height = map_height;
    blocks = new ClientMapBlock[width * height];
    
    int* map_data_iter = (int*)&map_data[0];
    ClientMapBlock* map_iter = blocks;
    
    for(int y = 0; y < height; y++)
        for(int x = 0; x < width; x++) {
            int data = *map_data_iter++;
            
            *map_iter++ = ClientMapBlock((BlockType)(data & 0xff), (LiquidType)(data >> 8 & 0xff), data >> 16 & 0xff, data >> 24 & 0xff);
        }

    view_x = map_width * BLOCK_WIDTH;
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

void ClientBlocks::updateOrientationDown(unsigned short x, unsigned short y) {
    getBlock(x, y).setState(getBlock(x, y).getState() * 2);
    if(updateOrientationSide(x, y, 0, 1))
        getBlock(x, y).setState(getBlock(x, y).getState() + 1);
}

void ClientBlocks::updateOrientationUp(unsigned short x, unsigned short y) {
    getBlock(x, y).setState(getBlock(x, y).getState() * 2);
    if(updateOrientationSide(x, y, 0, -1))
        getBlock(x, y).setState(getBlock(x, y).getState() + 1);
}

void ClientBlocks::updateOrientationLeft(unsigned short x, unsigned short y) {
    getBlock(x, y).setState(getBlock(x, y).getState() * 2);
    if(updateOrientationSide(x, y, -1, 0))
        getBlock(x, y).setState(getBlock(x, y).getState() + 1);
}

void ClientBlocks::updateOrientationRight(unsigned short x, unsigned short y) {
    getBlock(x, y).setState(getBlock(x, y).getState() * 2);
    if(updateOrientationSide(x, y, 1, 0))
        getBlock(x, y).setState(getBlock(x, y).getState() + 1);
}

bool ClientBlocks::updateOrientationSide(unsigned short x, unsigned short y, char side_x, char side_y) {
    if(
            x + side_x >= width || x + side_x < 0 || y + side_y >= height || y + side_y < 0 ||
            getBlock(x + side_x, y + side_y).getBlockType() == getBlock(x, y).getBlockType() ||
            std::count(getBlock(x, y).getBlockInfo().connects_to.begin(), getBlock(x, y).getBlockInfo().connects_to.end(), getBlock(x + side_x, y + side_y).getBlockType())
            )
        return true;
    else
        return false;
}
