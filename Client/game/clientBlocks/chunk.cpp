#include <cassert>
#include "clientBlocks.hpp"

void ClientBlocks::updateChunks() {
    short begin_x = view_x / (BLOCK_WIDTH * 32) - gfx::getWindowWidth() / 2 / (BLOCK_WIDTH * 32) - 1;
    short end_x = view_x / (BLOCK_WIDTH * 32) + gfx::getWindowWidth() / 2 / (BLOCK_WIDTH * 32) + 2;

    short begin_y = view_y / (BLOCK_WIDTH * 32) - gfx::getWindowHeight() / 2 / (BLOCK_WIDTH * 32) - 1;
    short end_y = view_y / (BLOCK_WIDTH * 32) + gfx::getWindowHeight() / 2 / (BLOCK_WIDTH * 32) + 2;

    if(begin_x < 0)
        begin_x = 0;
    if(end_x > getWorldWidth() >> 4)
        end_x = getWorldWidth() >> 4;
    if(begin_y < 0)
        begin_y = 0;
    if(end_y > getWorldHeight() >> 4)
        end_y = getWorldHeight() >> 4;


    for(unsigned short x = begin_x; x < end_x; x++)
        for(unsigned short y = begin_y; y < end_y; y++) {
            if(getChunk(x, y).state == ChunkState::unloaded) {
                sf::Packet packet;
                packet << PacketType::CHUNK << x << y;
                networking_manager->sendPacket(packet);
                getChunk(x, y).state = ChunkState::pending_load;
            }
        }
}

ClientMapChunk& ClientBlocks::getChunk(unsigned short x, unsigned short y) {
    assert(y >= 0 && y < (getWorldHeight() >> 4) && x >= 0 && x < (getWorldWidth() >> 4));
    return chunks[y * (getWorldWidth() >> 4) + x];
}
