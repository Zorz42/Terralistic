//
//  textures.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 28/06/2021.
//

#include "textures.hpp"
#include "assert.hpp"

static gfx::image block_textures[(int)blockType::NUM_BLOCKS],
item_textures[(int)itemType::NUM_ITEMS],
item_text_textures[(int)itemType::NUM_ITEMS],
liquid_textures[(int)liquidType::NUM_LIQUIDS],
breaking_texture;

const gfx::image& getBlockTexture(blockType type) {
    ASSERT((int)type >= 0 && type < blockType::NUM_BLOCKS, "block id is not valid")
    return block_textures[(int)type];
}

gfx::image& getItemTexture(itemType type) {
    ASSERT((int)type >= 0 && type < itemType::NUM_ITEMS, "item id is not valid")
    return item_textures[(int)type];
}

gfx::image& getItemTextTexture(itemType type) {
    ASSERT((int)type >= 0 && type < itemType::NUM_ITEMS, "item id is not valid")
    return item_text_textures[(int)type];
}

const gfx::image& getLiquidTexture(liquidType type) {
    ASSERT((int)type >= 0 && type < liquidType::NUM_LIQUIDS, "liquid id is not valid")
    return liquid_textures[(int)type];
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
    
    for(int i = 0; i < (int)itemType::NUM_ITEMS; i++) {
        gfx::image& texture = item_textures[i];
        texture.setTexture(getUniqueItem((itemType)i).name == "nothing" ? nullptr : gfx::loadImageFile("texturePack/items/" + getUniqueItem((itemType)i).name + ".png"));
        texture.scale = 2;
        
        texture = item_text_textures[i];
        texture.setTexture(gfx::renderText(getUniqueItem((itemType)i).name, {255, 255, 255}));
        texture.scale = 2;
    }
    
    for(int i = 0; i < (int)liquidType::NUM_LIQUIDS; i++) {
        gfx::image& texture = liquid_textures[i];
        texture.setTexture(gfx::loadImageFile("texturePack/liquids/" + getUniqueLiquid((liquidType)i).name + ".png"));
        texture.scale = 2;
    }
}
