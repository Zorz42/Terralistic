//
//  worldSelector.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#include <utility>
#include <vector>
#include <dirent.h>
#include <algorithm>
#include "worldSelector.hpp"
#include "game.hpp"
#include "worldCreator.hpp"
#include "fileSystem.hpp"

#undef main

// this is the menu where you select all the worlds you have

// every laoded world becomes a class/struct which is in array and rendered

#define PADDING 20
#define TOP_HEIGHT 70
#define BOTTOM_HEIGHT (back_button.getHeight() + PADDING + PADDING / 2)
#define LINE_HEIGHT 2

void worldSelector::world_to_select::render(int position_) {
    button.y = short(button_y - position_);
    gfx::render(button);
    delete_button.y = short(button_y - position_ + (button.getTranslatedRect().h - delete_button.getTranslatedRect().h) / 2);
    gfx::render(delete_button);
}

bool ends_with(const std::string& value, std::string ending) {
    return ending.size() <= value.size() && std::equal(ending.rbegin(), ending.rend(), value.rbegin());
}

void worldSelector::init() {
    // set some dimensions for shapes
    title.scale = 3;
    title.setTexture(gfx::renderText("Select a world to play!", {255, 255, 255}));
    title.y = PADDING;
    title.orientation = gfx::top;
    
    back_button.scale = 3;
    back_button.setTexture(gfx::renderText("Back", {255, 255, 255}));
    back_button.y = -PADDING;
    back_button.orientation = gfx::bottom;

    new_button.scale = 3;
    new_button.setTexture(gfx::renderText("New", {255, 255, 255}));
    new_button.y = -PADDING;
    new_button.x = -PADDING;
    new_button.orientation = gfx::bottom_right;
    
    x_image.setTexture(gfx::loadImageFile("texturePack/misc/x-button.png"));
}

void worldSelector::refresh() {
    // scans for worlds in world folder and sets their positions and renders them
    position = 0;
    scroll_limit = 0;
    
    worlds.clear();
    worlds_names.clear();

    std::vector<std::string> ignored_dirs = {
            ".",
            "..",
            ".DS_Store",
    };

    DIR *dir = opendir(fileSystem::getWorldsPath().c_str());
    dirent *ent;
    while((ent = readdir(dir)) != nullptr) {
        std::string name = ent->d_name;
        if(!std::count(ignored_dirs.begin(), ignored_dirs.end(), name) && ends_with(name, ".world"))
            worlds.emplace_back(name.substr(0, name.size()-6));
    }
    closedir (dir);
    
    for(auto& world : worlds) {
        world.button.orientation = gfx::top;
        world.button.scale = 3;
        world.button.setTexture(gfx::renderText(world.name, {255, 255, 255}));
        world.button_y = scroll_limit + title.getTranslatedRect().h + 2 * PADDING;
        
        world.delete_button.orientation = gfx::top;
        world.delete_button.free_texture = false;
        world.delete_button.setTexture(x_image.getTexture());
        world.delete_button.scale = 3;
        world.delete_button.x = short(world.button.getTranslatedRect().w / 2 + world.delete_button.getTranslatedRect().w / 2 + PADDING);
        
        scroll_limit += world.button.getTranslatedRect().h + PADDING;
        
        worlds_names.push_back(world.name);
    }
}

void worldSelector::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_MOUSE_LEFT) {
        if(back_button.isHovered())
            gfx::returnFromScene();
        else if(new_button.isHovered())
            gfx::switchScene(new worldCreator(worlds_names));
        else
            for(const world_to_select& i : worlds) {
                if(i.button.isHovered())
                    gfx::switchScene(new game(i.name, false));
                else if(i.delete_button.isHovered()) {
                    fileSystem::removeFile(fileSystem::getWorldsPath() + i.name + ".world");
                    refresh();
                }
            }
    }
}

void worldSelector::onMouseScroll(int distance) {
    position -= distance * 4;
    if(position < 0)
        position = 0;
    int scroll_limit_ = scroll_limit - gfx::getWindowHeight() + TOP_HEIGHT + BOTTOM_HEIGHT;
    if(position > scroll_limit_)
        position = scroll_limit_ > 0 ? scroll_limit_ : 0;
}

void worldSelector::render() {
    bool hoverable = gfx::getMouseY() > TOP_HEIGHT && gfx::getMouseY() < gfx::getWindowHeight() - BOTTOM_HEIGHT;
    
    for(world_to_select& world : worlds) {
        world.button.hoverable = hoverable;
        world.delete_button.hoverable = hoverable;
    }
    
    for(world_to_select& world : worlds)
        world.render(position);
    
    gfx::render(gfx::rect(0, 0, gfx::getWindowWidth(), TOP_HEIGHT, {0, 0, 0}));
    gfx::render(gfx::rect(0, 0, gfx::getWindowWidth(), BOTTOM_HEIGHT, {0, 0, 0}, gfx::bottom_left));
    gfx::render(gfx::rect(0, TOP_HEIGHT, gfx::getWindowWidth(), LINE_HEIGHT, {100, 100, 100}));
    gfx::render(gfx::rect(0, -BOTTOM_HEIGHT, gfx::getWindowWidth(), LINE_HEIGHT, {100, 100, 100}, gfx::bottom_left));
    
    gfx::render(title);
    gfx::render(back_button);
    gfx::render(new_button);
}
