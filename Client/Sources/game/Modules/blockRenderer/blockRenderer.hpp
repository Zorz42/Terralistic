//
//  blockRenderer.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 27/02/2021.
//

#ifndef blockRenderer_hpp
#define blockRenderer_hpp

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include "game.hpp"
#include "networkingModule.hpp"
#include "core.hpp"

struct blockRenderer : public gfx::sceneModule<game>, networking::packetListener, events::eventListener {
    blockRenderer(game* scene, networking::networkingManager* manager) : gfx::sceneModule<game>(scene), networking::packetListener(manager) {}
    void init() override;
    void render() override;
    void stop() override;
    void onPacket(packets::packet packet) override;
    void onEvent(events::eventType type, void* data) override;
    
private:
    struct uniqueBlock {
        void loadFromUniqueBlock(blockEngine::uniqueBlock* unique_block);
        gfx::image texture;
        std::vector<blockEngine::blockType> connects_to;
        bool single_texture;
    };

    struct block {
        unsigned char orientation{0};
        bool update = true;
    };

    struct chunk {
        bool update = true;
        gfx::image texture;
        void createTexture();
    };
    
    gfx::image breaking_texture;
    chunk* chunks = nullptr;
    block* blocks = nullptr;
    uniqueBlock* unique_blocks = nullptr;
    
    block& getBlock(unsigned short x, unsigned short y);
    chunk& getChunk(unsigned short x, unsigned short y);
    
    void renderBlock(unsigned short x, unsigned short y);
    void updateChunkTexture(unsigned short x, unsigned short y);
    void scheduleTextureUpdateForBlock(unsigned short x, unsigned short y);
    void renderChunk(unsigned short x, unsigned short y);
    void updateBlockOrientation(unsigned short x, unsigned short y);
    
    uniqueBlock& getUniqueBlock(unsigned short x, unsigned short y);
    
    void updateBlock(unsigned short x, unsigned short y);
};

#endif /* blockRenderer_hpp */
