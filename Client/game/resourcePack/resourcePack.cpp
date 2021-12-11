#include <fstream>
#include <filesystem>
#include "resourcePack.hpp"
#include "platform_folders.h"

const gfx::Texture& ResourcePack::getItemTexture() {
    return item_texture_atlas;
}

const gfx::Texture& ResourcePack::getItemTextTexture(ItemType* type) {
    return item_text_textures[(int)type->id];
}

const gfx::Texture& ResourcePack::getLiquidTexture() {
    return liquid_texture_atlas;
}

const gfx::Texture& ResourcePack::getPlayerTexture() {
    return player_texture;
}

const gfx::Texture& ResourcePack::getBackground() {
    return background;
}

const gfx::RectShape& ResourcePack::getTextureRectangle(LiquidType* type) {
    return liquid_texture_rectangles[(int)type->id];
}

const gfx::RectShape& ResourcePack::getTextureRectangle(ItemType* type) {
    return item_texture_rectangles[(int)type->id];
}

std::string ResourcePack::getFile(const std::string& file_name) {
    std::string file;
    for(int i = 0; i < paths.size(); i++) {
        file = paths[i] + file_name;
        if(std::filesystem::exists(file))
            return file;
    }
    throw Exception(file_name + " was not found.");
}

void ResourcePack::loadLiquids() {
    gfx::Texture *liquid_textures = new gfx::Texture[liquids->getNumLiquidTypes()];

    for(int i = 1; i < liquids->getNumLiquidTypes(); i++)
        liquid_textures[i].loadFromFile(getFile("/liquids/" + liquids->getLiquidTypeById(i)->name + ".png"));

    int max_y_size = 0;
    int texture_atlas_height = 0;
    for(int i = 1; i < liquids->getNumLiquidTypes(); i++){
        if(liquid_textures[i].getTextureWidth() > max_y_size)
            max_y_size = liquid_textures[i].getTextureWidth();
        liquid_texture_rectangles[i] = gfx::RectShape(0, texture_atlas_height, liquid_textures[i].getTextureWidth(), liquid_textures[i].getTextureHeight());
        texture_atlas_height += liquid_textures[i].getTextureHeight();
    }

    liquid_texture_atlas.createBlankImage(max_y_size, texture_atlas_height);
    gfx::setRenderTarget(liquid_texture_atlas);
    texture_atlas_height = 0;
    for(int i = 1; i < liquids->getNumLiquidTypes(); i++){
        liquid_textures[i].render(1, 0, texture_atlas_height);
        texture_atlas_height += liquid_textures[i].getTextureHeight();
    }
    gfx::resetRenderTarget();
}

void ResourcePack::loadItems() {
    gfx::Texture *item_textures = new gfx::Texture[items->getNumItemTypes()];

    for(int i = 1; i < items->getNumItemTypes(); i++) {
        item_textures[i].loadFromFile(getFile("/items/" + items->getItemTypeById(i)->name + ".png"));
        item_text_textures[i].loadFromText(items->getItemTypeById(i)->name);
    }

    int max_x_size = 0;
    int texture_atlas_height = 0;
    for(int i = 1; i < items->getNumItemTypes(); i++){
        if(item_textures[i].getTextureWidth() > max_x_size)
            max_x_size = item_textures[i].getTextureWidth();
        item_texture_rectangles[i] = gfx::RectShape(0, texture_atlas_height, item_textures[i].getTextureWidth(), item_textures[i].getTextureHeight());
        texture_atlas_height += item_textures[i].getTextureHeight();
    }

    item_texture_atlas.createBlankImage(max_x_size, texture_atlas_height);
    gfx::setRenderTarget(item_texture_atlas);
    texture_atlas_height = 0;
    for(int i = 1; i < items->getNumItemTypes(); i++){
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
    
    player_texture.loadFromFile(getFile("/misc/player.png"));
    background.loadFromFile(getFile("/misc/background.png"));

    item_text_textures = new gfx::Texture[items->getNumItemTypes()];
    liquid_texture_rectangles = new gfx::RectShape[liquids->getNumLiquidTypes()];
    item_texture_rectangles = new gfx::RectShape[items->getNumItemTypes()];
    loadLiquids();
    loadItems();
}

void ResourcePack::stop() {
    delete[] item_text_textures;
    delete[] liquid_texture_rectangles;
    delete[] item_texture_rectangles;
}
