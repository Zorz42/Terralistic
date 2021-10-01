#include "blockRenderer.hpp"
#include "platform_folders.h"

bool updateOrientationSide(Blocks* blocks, BlockRenderer* client_blocks, unsigned short x, unsigned short y, char side_x, char side_y) {
    if(
            x + side_x >= blocks->getWidth() || x + side_x < 0 || y + side_y >= blocks->getHeight() || y + side_y < 0 ||
            blocks->getBlockType(x + side_x, y + side_y) == blocks->getBlockType(x, y) ||
            std::count(blocks->getBlockInfo(x, y).connects_to.begin(), blocks->getBlockInfo(x, y).connects_to.end(), blocks->getBlockType(x + side_x, y + side_y))
            )
        return true;
    else
        return false;
}

void updateOrientationDown(Blocks* blocks, BlockRenderer* client_blocks, unsigned short x, unsigned short y) {
    client_blocks->setState(x, y, client_blocks->getState(x, y) * 2);
    if(updateOrientationSide(blocks, client_blocks, x, y, 0, 1))
        client_blocks->setState(x, y, client_blocks->getState(x, y) + 1);
}

void updateOrientationUp(Blocks* blocks, BlockRenderer* client_blocks, unsigned short x, unsigned short y) {
    client_blocks->setState(x, y, client_blocks->getState(x, y) * 2);
    if(updateOrientationSide(blocks, client_blocks, x, y, 0, -1))
        client_blocks->setState(x, y, client_blocks->getState(x, y) + 1);
}

void updateOrientationLeft(Blocks* blocks, BlockRenderer* client_blocks, unsigned short x, unsigned short y) {
    client_blocks->setState(x, y, client_blocks->getState(x, y) * 2);
    if(updateOrientationSide(blocks, client_blocks, x, y, -1, 0))
        client_blocks->setState(x, y, client_blocks->getState(x, y) + 1);
}

void updateOrientationRight(Blocks* blocks, BlockRenderer* client_blocks, unsigned short x, unsigned short y) {
    client_blocks->setState(x, y, client_blocks->getState(x, y) * 2);
    if(updateOrientationSide(blocks, client_blocks, x, y, 1, 0))
        client_blocks->setState(x, y, client_blocks->getState(x, y) + 1);
}

BlockRenderer::BlockRenderer(ResourcePack* resource_pack, NetworkingManager* manager, Blocks* blocks, Liquids* liquids, Lights* lights) : resource_pack(resource_pack), manager(manager), blocks(blocks), liquids(liquids), lights(lights) {
    stateFunctions[(int)BlockType::DIRT] = std::vector<void (*)(Blocks*, BlockRenderer*, unsigned short, unsigned short)>{&updateOrientationLeft, &updateOrientationDown, &updateOrientationRight, &updateOrientationUp};
    stateFunctions[(int)BlockType::STONE_BLOCK] = std::vector<void (*)(Blocks*, BlockRenderer*, unsigned short, unsigned short)>{&updateOrientationLeft, &updateOrientationDown, &updateOrientationRight, &updateOrientationUp};
    stateFunctions[(int)BlockType::GRASS_BLOCK] = std::vector<void (*)(Blocks*, BlockRenderer*, unsigned short, unsigned short)>{&updateOrientationLeft, &updateOrientationDown, &updateOrientationRight, &updateOrientationUp};
    stateFunctions[(int)BlockType::WOOD] = std::vector<void (*)(Blocks*, BlockRenderer*, unsigned short, unsigned short)>{&updateOrientationLeft, &updateOrientationDown, &updateOrientationRight, &updateOrientationUp};
    stateFunctions[(int)BlockType::LEAVES] = std::vector<void (*)(Blocks*, BlockRenderer*, unsigned short, unsigned short)>{&updateOrientationLeft, &updateOrientationDown, &updateOrientationRight, &updateOrientationUp};
    stateFunctions[(int)BlockType::SAND] = std::vector<void (*)(Blocks*, BlockRenderer*, unsigned short, unsigned short)>{&updateOrientationLeft, &updateOrientationDown, &updateOrientationRight, &updateOrientationUp};
    stateFunctions[(int)BlockType::SNOWY_GRASS_BLOCK] = std::vector<void (*)(Blocks*, BlockRenderer*, unsigned short, unsigned short)>{&updateOrientationLeft, &updateOrientationDown, &updateOrientationRight, &updateOrientationUp};
    stateFunctions[(int)BlockType::SNOW_BLOCK] = std::vector<void (*)(Blocks*, BlockRenderer*, unsigned short, unsigned short)>{&updateOrientationLeft, &updateOrientationDown, &updateOrientationRight, &updateOrientationUp};
    stateFunctions[(int)BlockType::ICE] = std::vector<void (*)(Blocks*, BlockRenderer*, unsigned short, unsigned short)>{&updateOrientationLeft, &updateOrientationDown, &updateOrientationRight, &updateOrientationUp};
}

void BlockRenderer::init() {
    manager->packet_event.addListener(this);
    view_x = blocks->getWidth() * BLOCK_WIDTH;
    view_y = 0;
}

void BlockRenderer::create() {
    client_blocks = new RenderBlocks[blocks->getWidth() * blocks->getHeight()];
}

BlockRenderer::RenderBlocks* BlockRenderer::getClientBlock(unsigned short x, unsigned short y) {
    return &client_blocks[y * blocks->getWidth() + x];
}

void BlockRenderer::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case PacketType::BLOCK: {
            unsigned short x, y;
            unsigned char block_type;
            event.packet >> x >> y >> block_type;
            
            blocks->setBlockType(x, y, (BlockType)block_type);
            break;
        }
        case PacketType::LIQUID: {
            unsigned short x, y;
            unsigned char liquid_type, liquid_level;
            event.packet >> x >> y >> liquid_type >> liquid_level;
            
            liquids->setLiquidType(x, y, (LiquidType)liquid_type);
            liquids->setLiquidLevel(x, y, liquid_level);
            
            break;
        }
        case PacketType::BLOCK_PROGRESS: {
            unsigned short x, y, progress;
            event.packet >> x >> y >> progress;
            
            blocks->setBreakProgress(x, y, progress);
            break;
        }
        default:;
    }
}

BlockRenderer::~BlockRenderer() {
    delete[] client_blocks;
}

void BlockRenderer::updateLights() {
    bool finished = false;
    while(!finished) {
        finished = true;
        for(int y = getViewBeginY(); y < getViewEndY(); y++)
            for(int x = getViewBeginX(); x < getViewEndX(); x++)
                if(lights->hasScheduledLightUpdate(x, y)) {
                    lights->updateLight(x, y);
                    finished = true;
                }
    }
}
 
