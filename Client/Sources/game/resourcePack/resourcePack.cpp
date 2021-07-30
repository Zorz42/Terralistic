#include <cassert>
#include "resourcePack.hpp"

const gfx::Image& ResourcePack::getBlockTexture(BlockType type) {
    assert((int)type >= 0 && type < BlockType::NUM_BLOCKS);
    return block_textures[(int)type];
}

const gfx::Image& ResourcePack::getItemTexture(ItemType type) {
    assert((int)type >= 0 && type < ItemType::NUM_ITEMS);
    return item_textures[(int)type];
}

const gfx::Image& ResourcePack::getItemTextTexture(ItemType type) {
    assert((int)type >= 0 && type < ItemType::NUM_ITEMS);
    return item_text_textures[(int)type];
}

const gfx::Image& ResourcePack::getLiquidTexture(LiquidType type) {
    assert((int)type >= 0 && type < LiquidType::NUM_LIQUIDS);
    return liquid_textures[(int)type];
}

const gfx::Image& ResourcePack::getBreakingTexture() {
    return breaking_texture;
}

const gfx::Image& ResourcePack::getPlayerTexture() {
    return player_texture;
}

const gfx::Image& ResourcePack::getBackground() {
    return background;
}

void ResourcePack::load(std::string path) {
    breaking_texture.loadFromFile(path + "/misc/breaking.png");
    player_texture.loadFromFile(path + "/misc/player.png");
    background.loadFromFile(path + "/misc/background.png");
    
    for(int i = 0; i < (int)BlockType::NUM_BLOCKS; i++)
        block_textures[i].loadFromFile(path + "/blocks/" + getBlockInfo((BlockType)i).name + ".png");
    
    for(int i = 0; i < (int)ItemType::NUM_ITEMS; i++) {
        item_textures[i].loadFromFile(path + "/items/" + getItemInfo((ItemType)i).name + ".png");
        
        item_text_textures[i].renderText(getItemInfo((ItemType)i).name);
    }
    
    for(int i = 0; i < (int)LiquidType::NUM_LIQUIDS; i++)
        liquid_textures[i].loadFromFile(path + "/liquids/" + getLiquidInfo((LiquidType)i).name + ".png");
}
