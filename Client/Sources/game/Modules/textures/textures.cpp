//
//  textures.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 28/06/2021.
//

#include "textures.hpp"

static gfx::image block_textures[(int)blockType::NUM_BLOCKS], breaking_texture;

const gfx::image& getBlockTexture(blockType type) {
    return block_textures[(int)type];
}

const gfx::image& getBreakingTexture() {
    return breaking_texture;
}

void loadTextures() {
    breaking_texture.setTexture(gfx::loadImageFile("texturePack/misc/breaking.png"));
    breaking_texture.scale = 2;
    
    for(int i = 0; i < (int)blockType::NUM_BLOCKS; i++) {
        gfx::image& texture = block_textures[i];
        texture.setTexture(gfx::loadImageFile("texturePack/blocks/" + getUniqueBlock((blockType)i).name + ".png"));
        texture.scale = 2;
    }
}
