//
//  blockEngineNet.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/02/2021.
//

#define FILENAME blockEngineNet
#define NAMESPACE networkingModule
#include "core.hpp"

#include "networkingModule.hpp"
#include "blockRenderer.hpp"

PACKET_LISTENER(packets::BLOCK_CHANGE)
    auto type = (blockEngine::blockType)packet.getUChar();
    unsigned short y = packet.getUShort(), x = packet.getUShort();
    blockEngine::removeNaturalLight(x);
    blockEngine::getBlock(x, y).setBlockType(type);
    blockEngine::setNaturalLight(x);
    blockEngine::getBlock(x, y).light_update();
PACKET_LISTENER_END

PACKET_LISTENER(packets::CHUNK)
    unsigned short x = packet.getUShort(), y = packet.getUShort();
    blockEngine::chunk& chunk = blockEngine::getChunk(x, y);
    for(unsigned short x_ = x << 4; x_ < (x << 4) + 16; x_++)
        blockEngine::removeNaturalLight(x_);
    for(unsigned short x_ = 0; x_ < 16; x_++)
        for(unsigned short y_ = 0; y_ < 16; y_++)
            blockEngine::getBlock(x + x_, y + y_).setBlockType((blockEngine::blockType)packet.getChar());
    for(unsigned short x_ = x << 4; x_ < (x << 4) + 16; x_++)
        blockEngine::setNaturalLight(x_);
    chunk.loaded = true;
PACKET_LISTENER_END

PACKET_LISTENER(packets::BLOCK_BREAK_PROGRESS_CHANGE)
    unsigned short progress = packet.getUShort(), x = packet.getUShort(), y = packet.getUShort();
    blockEngine::getBlock(x, y).break_progress = progress;
PACKET_LISTENER_END

