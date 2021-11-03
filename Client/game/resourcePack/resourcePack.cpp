#include <cassert>
#include <filesystem>
#include <fstream>
#include "print.hpp"
#include "resourcePack.hpp"
#include "platform_folders.h"

const gfx::Texture& ResourcePack::getBlockTexture(){
    return block_texture_atlas;
}

const gfx::Texture& ResourcePack::getItemTexture(){
    return item_texture_atlas;
}

const gfx::Texture& ResourcePack::getItemTextTexture(ItemTypeOld type) {
    assert((int)type >= 0 && type < ItemTypeOld::NUM_ITEMS);
    return item_text_textures[(int)type];
}

const gfx::Texture& ResourcePack::getLiquidTexture() {
    return liquid_texture_atlas;
}

const gfx::Texture& ResourcePack::getBreakingTexture() {
    return breaking_texture;
}

const gfx::Texture& ResourcePack::getPlayerTexture() {
    return player_texture;
}

const gfx::Texture& ResourcePack::getBackground() {
    return background;
}

const gfx::RectShape& ResourcePack::getTextureRectangle(BlockTypeOld type) {
    return block_texture_rectangles[(int)type];
}

const gfx::RectShape& ResourcePack::getTextureRectangle(LiquidTypeOld type) {
    return liquid_texture_rectangles[(int)type];
}

const gfx::RectShape& ResourcePack::getTextureRectangle(ItemTypeOld type) {
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

void ResourcePack::loadBlocks() {
    gfx::Texture block_textures[(int)BlockTypeOld::NUM_BLOCKS];

    for(int i = 1; i < (int)BlockTypeOld::NUM_BLOCKS; i++)
        block_textures[i].loadFromFile(getFile("/blocks/" + getBlockInfoOld((BlockTypeOld)i).name + ".png"));

    unsigned short max_y_size = 0;
    int texture_atlas_height = 0;
    for(int i = 1; i < (int)BlockTypeOld::NUM_BLOCKS; i++){
        if(block_textures[i].getTextureWidth() > max_y_size)
            max_y_size = block_textures[i].getTextureWidth();
        block_texture_rectangles[i] = gfx::RectShape(0, texture_atlas_height, block_textures[i].getTextureWidth(), block_textures[i].getTextureHeight());
        texture_atlas_height += block_textures[i].getTextureHeight();
    }

    block_texture_atlas.createBlankImage(max_y_size, texture_atlas_height);
    gfx::setRenderTarget(block_texture_atlas);
    texture_atlas_height = 0;
    for(int i = 1; i < (int)BlockTypeOld::NUM_BLOCKS; i++){
        block_textures[i].render(1, 0, texture_atlas_height);
        texture_atlas_height += block_textures[i].getTextureHeight();
    }
    gfx::resetRenderTarget();
}

void ResourcePack::loadLiquids() {
    gfx::Texture liquid_textures[(int)LiquidTypeOld::NUM_LIQUIDS];

    for(int i = 1; i < (int)LiquidTypeOld::NUM_LIQUIDS; i++)
        liquid_textures[i].loadFromFile(getFile("/liquids/" + getLiquidInfoOld((LiquidTypeOld)i).name + ".png"));

    unsigned short max_y_size = 0;
    int texture_atlas_height = 0;
    for(int i = 1; i < (int)LiquidTypeOld::NUM_LIQUIDS; i++){
        if(liquid_textures[i].getTextureWidth() > max_y_size)
            max_y_size = liquid_textures[i].getTextureWidth();
        liquid_texture_rectangles[i] = gfx::RectShape(0, texture_atlas_height, liquid_textures[i].getTextureWidth(), liquid_textures[i].getTextureHeight());
        texture_atlas_height += liquid_textures[i].getTextureHeight();
    }

    liquid_texture_atlas.createBlankImage(max_y_size, texture_atlas_height);
    gfx::setRenderTarget(liquid_texture_atlas);
    texture_atlas_height = 0;
    for(int i = 1; i < (int)LiquidTypeOld::NUM_LIQUIDS; i++){
        liquid_textures[i].render(1, 0, texture_atlas_height);
        texture_atlas_height += liquid_textures[i].getTextureHeight();
    }
    gfx::resetRenderTarget();
}

void ResourcePack::loadItems() {
    gfx::Texture item_textures[(int)ItemTypeOld::NUM_ITEMS];

    for(int i = 1; i < (int)ItemTypeOld::NUM_ITEMS; i++) {
        item_textures[i].loadFromFile(getFile("/items/" + getItemInfoOld((ItemTypeOld)i).name + ".png"));
        item_text_textures[i].loadFromText(getItemInfoOld((ItemTypeOld)i).name);
    }

    unsigned short max_x_size = 0;
    int texture_atlas_height = 0;
    for(int i = 1; i < (int)ItemTypeOld::NUM_ITEMS; i++){
        if(item_textures[i].getTextureWidth() > max_x_size)
            max_x_size = item_textures[i].getTextureWidth();
        item_texture_rectangles[i] = gfx::RectShape(0, texture_atlas_height, item_textures[i].getTextureWidth(), item_textures[i].getTextureHeight());
        texture_atlas_height += item_textures[i].getTextureHeight();
    }

    item_texture_atlas.createBlankImage(max_x_size, texture_atlas_height);
    gfx::setRenderTarget(item_texture_atlas);
    texture_atlas_height = 0;
    for(int i = 1; i < (int)ItemTypeOld::NUM_ITEMS; i++){
        item_textures[i].render(1, 0, texture_atlas_height);
        texture_atlas_height += item_textures[i].getTextureHeight();
    }
    gfx::resetRenderTarget();
}

void ResourcePack::init() {
    std::vector<std::string> active_resource_packs = {gfx::getResourcePath() + "resourcePack"};
    if(std::filesystem::exists(sago::getDataHome() + "/Terralistic/activeMods.txt")) {
        std::ifstream active_mods_file(sago::getDataHome() + "/Terralistic/activeMods.txt");
        std::string line;
        while(std::getline(active_mods_file, line))
            active_resource_packs.insert(active_resource_packs.begin(), sago::getDataHome() + "/Terralistic/Mods/" + line);
    }
    
    paths = active_resource_packs;
    std::filesystem::create_directory(sago::getDataHome() + "/Terralistic/Mods/");

    breaking_texture.loadFromFile(getFile("/misc/breaking.png"));
    player_texture.loadFromFile(getFile("/misc/player.png"));
    background.loadFromFile(getFile("/misc/background.png"));

    loadBlocks();
    loadLiquids();
    loadItems();
}
