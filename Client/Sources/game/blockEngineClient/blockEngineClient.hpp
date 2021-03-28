//
//  blockRenderer.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 27/02/2021.
//

#ifndef blockRenderer_hpp
#define blockRenderer_hpp

#include <Graphics/graphics.hpp>
#include "game.hpp"
#include "networkingModule.hpp"

namespace blockEngineClient {

struct uniqueRenderBlock {
    void createTexture(blockEngine::uniqueBlock* unique_block);
    gfx::image texture;
    std::vector<blockEngine::blockType> connects_to;
    bool single_texture;
};

struct renderBlock {
    void updateOrientation();
    void draw();
    unsigned char block_orientation{0};
    bool to_update = true;
    void scheduleTextureUpdate();
    unsigned short getX() const;
    unsigned short getY() const;
    blockEngine::block& getRelatedBlock();
    uniqueRenderBlock& getUniqueRenderBlock();
    blockEngine::uniqueBlock& getUniqueBlock();
};

struct renderChunk {
    void render() const;
    bool update = true;
    gfx::image texture;
    void createTexture();
    void updateTexture();
    unsigned short getX() const;
    unsigned short getY() const;
};

void prepare();
void render();
void close();

struct module : public gfx::sceneModule<game::scene>, networking::packetListener {
    using gfx::sceneModule<game::scene>::sceneModule;
    void init();
    void render();
    void stop();
    void onPacket(packets::packet packet);
};

}

#endif /* blockRenderer_hpp */
