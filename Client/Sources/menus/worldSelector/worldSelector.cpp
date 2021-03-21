//
//  worldSelector.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#include "core.hpp"

#include <utility>
#include <vector>
#include <dirent.h>
#include <algorithm>
#include "worldSelector.hpp"
#include "gameLoop.hpp"
#include "worldCreator.hpp"

#undef main

// this is the menu where you select all the worlds you have

// every laoded world becomes a class/struct which is in array and rendered

struct world_to_select {
    std::string name;
    explicit world_to_select(std::string name) : name(std::move(name)) {}
    gfx::button button, delete_button;
    void render(bool display_hover);
    int button_y{};
};

#define PADDING 20
#define TOP_HEIGHT 70
#define BOTTOM_HEIGHT (back_button.getHeight() + PADDING + PADDING / 2)
#define LINE_HEIGHT 2

static gfx::sprite title;
static gfx::image x_image;
static gfx::button back_button, new_button;
static gfx::rect top_rect(0, 0, 0, TOP_HEIGHT, {0, 0, 0}), bottom_rect(0, 0, 0, 0, {0, 0, 0}, gfx::bottom_left), top_line_rect(0, TOP_HEIGHT, 0, LINE_HEIGHT, {100, 100, 100}), bottom_line_rect(0, 0, 0, LINE_HEIGHT, {100, 100, 100}, gfx::bottom_left);
static std::vector<std::string> worlds_names;
static std::vector<world_to_select> worlds;
static int scroll_limit, position;

void world_to_select::render(bool display_hover) {
    button.y = short(button_y - position);
    gfx::render(button);
    delete_button.y = short(button_y - position + (button.getTranslatedRect().h - delete_button.getTranslatedRect().h) / 2);
    gfx::render(delete_button);
}

INIT_SCRIPT
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
    
    bottom_rect.h = BOTTOM_HEIGHT;
    bottom_line_rect.y = -BOTTOM_HEIGHT;
    
    x_image.setTexture(gfx::loadImageFile("texturePack/misc/x-button.png"));
INIT_SCRIPT_END

bool ends_with(const std::string& value, std::string ending) {
    return ending.size() <= value.size() && std::equal(ending.rbegin(), ending.rend(), value.rbegin());
}

bool display_hover;

/*void worldSelector::loop() {
    bool running = true;
    SDL_Event event;
    
    reload();
    
    while(running && main_::running) {
        while(SDL_PollEvent(&event)) {
            if(swl::handleBasicEvents(event, &main_::running));
            else if(back_button.isPressed(event))
                running = false;
            else if(new_button.isPressed(event)) {
                worldCreator::loop(worlds_names);
                reload();
            }
            else if(event.type == SDL_MOUSEWHEEL) { // scrolling
                position -= event.wheel.y * 4;
                if(position < 0)
                    position = 0;
                int scroll_limit_ = scroll_limit - swl::window_height + TOP_HEIGHT + BOTTOM_HEIGHT;
                if(position > scroll_limit_)
                    position = scroll_limit_ > 0 ? scroll_limit_ : 0;
            }
            else
                for(world_to_select& world : worlds)
                    if(world.button.isPressed(event)) {
                        gameLoop::main(world.name, false);
                        reload();
                        break;
                    } else if(world.delete_button.isPressed(event)) {
                        fileSystem::removeFile(fileSystem::worlds_dir + world.name + ".world");
                        reload();
                    }
        }
        
        display_hover = swl::mouse_y > TOP_HEIGHT && swl::mouse_y < swl::window_height - BOTTOM_HEIGHT;
        
        top_rect.setWidth(swl::window_width);
        bottom_rect.setWidth(swl::window_width);
        top_line_rect.setWidth(swl::window_width);
        bottom_line_rect.setWidth(swl::window_width);
        
        swl::setDrawColor(0, 0, 0);
        swl::clear();
        
        for(world_to_select& world : worlds)
            world.render(display_hover);
        
        top_rect.render();
        bottom_rect.render();
        title.render();
        back_button.render();
        new_button.render();
        top_line_rect.render();
        bottom_line_rect.render();
        
        swl::update();
    }
}*/

void worldSelector::scene::refresh() {
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

    DIR *dir = opendir(fileSystem::worlds_dir.c_str());
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

void worldSelector::scene::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_MOUSE_LEFT) {
        if(back_button.isHovered())
            gfx::returnFromScene();
        else if(new_button.isHovered())
            gfx::switchScene(new worldCreator::scene(worlds_names));
        else
            for(const world_to_select& i : worlds) {
                if(i.button.isHovered())
                    gfx::switchScene(new gameLoop::scene(i.name, false));
                else if(i.delete_button.isHovered()) {
                    fileSystem::removeFile(fileSystem::worlds_dir + i.name + ".world");
                    refresh();
                }
            }
    }
}

void worldSelector::scene::render() {
    display_hover = gfx::getMouseY() > TOP_HEIGHT && gfx::getMouseY() < gfx::getWindowHeight() - BOTTOM_HEIGHT;
    
    for(world_to_select& world : worlds)
        world.render(display_hover);
    
    top_rect.w = gfx::getWindowWidth();
    bottom_rect.w = gfx::getWindowWidth();
    top_line_rect.w = gfx::getWindowWidth();
    bottom_line_rect.w = gfx::getWindowWidth();
    
    gfx::render(top_rect);
    gfx::render(bottom_rect);
    gfx::render(top_line_rect);
    gfx::render(bottom_line_rect);
    gfx::render(title);
    gfx::render(back_button);
    gfx::render(new_button);
}
