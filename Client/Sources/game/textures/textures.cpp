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

const gfx::image& getItemTexture(itemType type) {
    ASSERT((int)type >= 0 && type < itemType::NUM_ITEMS, "item id is not valid")
    return item_textures[(int)type];
}

const gfx::image& getItemTextTexture(itemType type) {
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
    
    for(int i = 0; i < (int)blockType::NUM_BLOCKS; i++)
        block_textures[i].setTexture(gfx::loadImageFile("texturePack/blocks/" + getBlockInfo((blockType)i).name + ".png"));
    
    for(int i = 0; i < (int)itemType::NUM_ITEMS; i++) {
        item_textures[i].setTexture(getItemInfo((itemType)i).name == "nothing" ? nullptr : gfx::loadImageFile("texturePack/items/" + getItemInfo((itemType)i).name + ".png"));
        
        item_text_textures[i].setTexture(gfx::renderText(getItemInfo((itemType)i).name, {255, 255, 255}));
    }
    
    for(int i = 0; i < (int)liquidType::NUM_LIQUIDS; i++)
        liquid_textures[i].setTexture(gfx::loadImageFile("texturePack/liquids/" + getLiquidInfo((liquidType)i).name + ".png"));
}
