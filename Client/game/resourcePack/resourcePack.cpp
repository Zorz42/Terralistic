#include <cassert>
#include <filesystem>
#include "print.hpp"
#include "resourcePack.hpp"
#include "platform_folders.h"

const gfx::Image& ResourcePack::getBlockTexture(){
    return block_texture_atlas;
}

const gfx::Image& ResourcePack::getItemTexture(){
    return item_texture_atlas;
}

const gfx::Image& ResourcePack::getItemTextTexture(ItemType type) {
    assert((int)type >= 0 && type < ItemType::NUM_ITEMS);
    return item_text_textures[(int)type];
}

const gfx::Image& ResourcePack::getLiquidTexture() {
    return liquid_texture_atlas;
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

const gfx::RectShape& ResourcePack::getTextureRectangle(BlockType type) {
    return block_texture_rectangles[(int)type];
}

const gfx::RectShape& ResourcePack::getTextureRectangle(LiquidType type) {
    return liquid_texture_rectangles[(int)type];
}

const gfx::RectShape& ResourcePack::getTextureRectangle(ItemType type) {
    return item_texture_rectangles[(int)type];
}

std::string ResourcePack::getFile(const std::string& file_name) {
    std::string file;
    for(const std::string& path : paths) {
        file = path + file_name;
        if(std::filesystem::exists(file))
            return file;
    }
    assert(false);
    return "";
}

void ResourcePack::load(const std::vector<std::string>& paths_) {
    paths = paths_;
    std::filesystem::create_directory(sago::getDataHome() + "/Terralistic/Mods/");

    breaking_texture.loadFromFile(getFile("/misc/breaking.png"));
    player_texture.loadFromFile(getFile("/misc/player.png"));
    background.loadFromFile(getFile("/misc/background.png"));

    loadBlocks();
    loadLiquids();
    loadItems();

}

void ResourcePack::loadBlocks() {
    gfx::Image block_textures[(int)BlockType::NUM_BLOCKS];

    for(int i = 1; i < (int)BlockType::NUM_BLOCKS; i++)
        block_textures[i].loadFromFile(getFile("/blocks/" + getBlockInfo((BlockType)i).name + ".png"));

    unsigned short max_y_size = 0;
    int texture_atlas_height = 0;
    for(int i = 1; i < (int)BlockType::NUM_BLOCKS; i++){
        if(block_textures[i].getTextureWidth() > max_y_size)
            max_y_size = block_textures[i].getTextureWidth();
        block_texture_rectangles[i] = gfx::RectShape(0, texture_atlas_height, block_textures[i].getTextureWidth(), block_textures[i].getTextureHeight());
        texture_atlas_height += block_textures[i].getTextureHeight();
    }

    block_texture_atlas.createBlankImage(max_y_size, texture_atlas_height);
    gfx::setRenderTarget(block_texture_atlas);
    texture_atlas_height = 0;
    for(int i = 1; i < (int)BlockType::NUM_BLOCKS; i++){
        block_textures[i].render(1, 0, texture_atlas_height);
        texture_atlas_height += block_textures[i].getTextureHeight();
    }
    gfx::resetRenderTarget();
}

void ResourcePack::loadLiquids() {
    gfx::Image liquid_textures[(int)LiquidType::NUM_LIQUIDS];

    for(int i = 1; i < (int)LiquidType::NUM_LIQUIDS; i++)
        liquid_textures[i].loadFromFile(getFile("/liquids/" + getLiquidInfo((LiquidType)i).name + ".png"));

    unsigned short max_y_size = 0;
    int texture_atlas_height = 0;
    for(int i = 1; i < (int)LiquidType::NUM_LIQUIDS; i++){
        if(liquid_textures[i].getTextureWidth() > max_y_size)
            max_y_size = liquid_textures[i].getTextureWidth();
        liquid_texture_rectangles[i] = gfx::RectShape(0, texture_atlas_height, liquid_textures[i].getTextureWidth(), liquid_textures[i].getTextureHeight());
        texture_atlas_height += liquid_textures[i].getTextureHeight();
    }

    liquid_texture_atlas.createBlankImage(max_y_size, texture_atlas_height);
    gfx::setRenderTarget(liquid_texture_atlas);
    texture_atlas_height = 0;
    for(int i = 1; i < (int)LiquidType::NUM_LIQUIDS; i++){
        liquid_textures[i].render(1, 0, texture_atlas_height);
        texture_atlas_height += liquid_textures[i].getTextureHeight();
    }
    gfx::resetRenderTarget();
}

void ResourcePack::loadItems() {
    gfx::Image item_textures[(int)ItemType::NUM_ITEMS];

    for(int i = 1; i < (int)ItemType::NUM_ITEMS; i++) {
        item_textures[i].loadFromFile(getFile("/items/" + getItemInfo((ItemType)i).name + ".png"));
        item_text_textures[i].loadFromText(getItemInfo((ItemType)i).name);
    }

    unsigned short max_x_size = 0;
    int texture_atlas_height = 0;
    for(int i = 1; i < (int)ItemType::NUM_ITEMS; i++){
        if(item_textures[i].getTextureWidth() > max_x_size)
            max_x_size = item_textures[i].getTextureWidth();
        item_texture_rectangles[i] = gfx::RectShape(0, texture_atlas_height, item_textures[i].getTextureWidth(), item_textures[i].getTextureHeight());
        texture_atlas_height += item_textures[i].getTextureHeight();
    }

    item_texture_atlas.createBlankImage(max_x_size, texture_atlas_height);
    gfx::setRenderTarget(item_texture_atlas);
    texture_atlas_height = 0;
    for(int i = 1; i < (int)ItemType::NUM_ITEMS; i++){
        item_textures[i].render(1, 0, texture_atlas_height);
        texture_atlas_height += item_textures[i].getTextureHeight();
    }
    gfx::resetRenderTarget();
}
