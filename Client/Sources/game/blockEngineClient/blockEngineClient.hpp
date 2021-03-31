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

namespace blockEngineClient {

struct module : public gfx::sceneModule<game::scene>, networking::packetListener {
    using gfx::sceneModule<game::scene>::sceneModule;
    void init();
    void render();
    void stop();
    void onPacket(packets::packet packet);
    
private:
    struct uniqueRenderBlock {
        void createTexture(blockEngine::uniqueBlock* unique_block);
        gfx::image texture;
        std::vector<blockEngine::blockType> connects_to;
        bool single_texture;
    };

    struct renderBlock {
        unsigned char block_orientation{0};
        bool to_update = true;
    };

    struct renderChunk {
        bool update = true;
        gfx::image texture;
        void createTexture();
        void updateTexture(module* block_engine);
    };
    
    gfx::image breaking_texture;
    renderChunk* chunks = nullptr;
    renderBlock* blocks = nullptr;
    uniqueRenderBlock* unique_render_blocks = nullptr;
    
    renderBlock& getBlock(unsigned short x, unsigned short y);
    renderChunk& getChunk(unsigned short x, unsigned short y);
    
    void drawRenderBlock(unsigned short x, unsigned short y);
    void updateChunkTexture(unsigned short x, unsigned short y);
    void scheduleTextureUpdate(unsigned short x, unsigned short y);
    void drawRenderChunk(unsigned short x, unsigned short y);
    void updateRenderBlockOrientation(unsigned short x, unsigned short y);
    
    uniqueRenderBlock& getUniqueRenderBlock(unsigned short x, unsigned short y);
    
    void updateBlock(unsigned short x, unsigned short y);
};

}

#endif /* blockRenderer_hpp */
