#include <cassert>
#include "resourcePack.hpp"

const gfx::Image& ResourcePack::getBlockTexture(){
    return texture_atlas;
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
    return texture_atlas;
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

    unsigned short max_y_size = 8;
    int texture_atlas_height = 0;
    for(int i = 0; i < (int)BlockType::NUM_BLOCKS; i++){
        //if(block_textures[i].getTextureWidth() > max_y_size)
            //max_y_size = block_textures[i].getTextureWidth();
        //block_infos[i].unique_textures = block_textures[i].getTextureWidth() / 8;
        block_infos[i].y_on_text_atlas = 16;//texture_atlas_height;
        texture_atlas_height += block_textures[i].getTextureHeight();
    }

    texture_atlas.createBlankImage(max_y_size, texture_atlas_height);
    gfx::setRenderTarget(texture_atlas);
    texture_atlas_height = 0;
    for(int i = 0; i < (int)BlockType::NUM_BLOCKS; i++){
        block_textures[i].render(1, 0, texture_atlas_height);
        texture_atlas_height += block_textures[i].getTextureHeight();
    }
    gfx::resetRenderTarget();




}
