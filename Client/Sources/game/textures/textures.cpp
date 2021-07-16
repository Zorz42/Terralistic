//
//  textures.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 28/06/2021.
//

#include "textures.hpp"
#include <cassert>

static gfx::Image block_textures[(int)BlockType::NUM_BLOCKS],
item_textures[(int)ItemType::NUM_ITEMS],
item_text_textures[(int)ItemType::NUM_ITEMS],
liquid_textures[(int)LiquidType::NUM_LIQUIDS],
breaking_texture;

const gfx::Image& getBlockTexture(BlockType type) {
    assert((int)type >= 0 && type < BlockType::NUM_BLOCKS);
    return block_textures[(int)type];
}

const gfx::Image& getItemTexture(ItemType type) {
    assert((int)type >= 0 && type < ItemType::NUM_ITEMS);
    return item_textures[(int)type];
}

const gfx::Image& getItemTextTexture(ItemType type) {
    assert((int)type >= 0 && type < ItemType::NUM_ITEMS);
    return item_text_textures[(int)type];
}

const gfx::Image& getLiquidTexture(LiquidType type) {
    assert((int)type >= 0 && type < LiquidType::NUM_LIQUIDS);
    return liquid_textures[(int)type];
}

const gfx::Image& getBreakingTexture() {
    return breaking_texture;
}

void loadTextures() {
    breaking_texture.loadFromFile("texturePack/misc/breaking.png");
    
    for(int i = 0; i < (int)BlockType::NUM_BLOCKS; i++)
        block_textures[i].loadFromFile("texturePack/blocks/" + getBlockInfo((BlockType)i).name + ".png");
    
    for(int i = 0; i < (int)ItemType::NUM_ITEMS; i++) {
        item_textures[i].loadFromFile("texturePack/items/" + getItemInfo((ItemType)i).name + ".png");
        
        item_text_textures[i].renderText(getItemInfo((ItemType)i).name, {255, 255, 255});
    }
    
    for(int i = 0; i < (int)LiquidType::NUM_LIQUIDS; i++)
        liquid_textures[i].loadFromFile("texturePack/liquids/" + getLiquidInfo((LiquidType)i).name + ".png");
}
