//
//  blockRenderer.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 27/02/2021.
//

#ifndef blockRenderer_hpp
#define blockRenderer_hpp

#include <Graphics/graphics.hpp>

namespace blockRenderer {

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

inline renderChunk* chunks;
inline renderBlock* blocks;
inline uniqueRenderBlock* unique_render_blocks;

renderBlock& getBlock(unsigned short x, unsigned short y);
renderChunk& getChunk(unsigned short x, unsigned short y);

void prepare();
void render();
void close();

}

#endif /* blockRenderer_hpp */
