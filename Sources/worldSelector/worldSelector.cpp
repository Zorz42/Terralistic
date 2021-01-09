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
#include "singleWindowLibrary.hpp"
#include "UIKit.hpp"
#include "framerateRegulator.hpp"
#include "gameLoop.hpp"
#include "fileSystem.hpp"
#include "worldCreator.hpp"

#undef main

struct world_to_select {
    std::string name;
    explicit world_to_select(std::string name) : name(std::move(name)) {}
    ui::button button{ogl::top}, delete_button{ogl::top};
    void render(bool display_hover);
    int button_y{};
};

ogl::texture title(ogl::top);
SDL_Texture* x_texture = nullptr;
unsigned short x_width, x_height;
ui::button back_button(ogl::bottom), new_button(ogl::bottom_right);
ogl::rect top_rect(ogl::top), bottom_rect(ogl::bottom), top_line_rect(ogl::top), bottom_line_rect(ogl::bottom);
int position;
std::vector<std::string> worlds_names;
std::vector<world_to_select> worlds;
int scroll_limit;

void world_to_select::render(bool display_hover) {
    button.y = short(button_y - position);
    button.render(display_hover);
    delete_button.y = short(button_y - position + (button.getHeight() - delete_button.getHeight()) / 2);
    delete_button.render(display_hover);
}

#define PADDING 20
#define TOP_HEIGHT 70
#define BOTTOM_HEIGHT (back_button.getHeight() + PADDING + PADDING / 2)
#define LINE_HEIGHT 2

void worldSelector::init() {
    title.scale = 3;
    title.loadFromText("Select a world to play!", {255, 255, 255});
    title.setY(PADDING);
    
    back_button.setColor(0, 0, 0);
    back_button.setHoverColor(100, 100, 100);
    back_button.setScale(3);
    back_button.setText("Back", 255, 255, 255);
    back_button.y = -PADDING;
    
    new_button.setColor(0, 0, 0);
    new_button.setHoverColor(100, 100, 100);
    new_button.setScale(3);
    new_button.setText("New", 255, 255, 255);
    new_button.y = -PADDING;
    new_button.x = -PADDING;

    top_rect.setHeight(TOP_HEIGHT);
    top_rect.setColor(0, 0, 0);
    
    bottom_rect.setHeight((unsigned short)BOTTOM_HEIGHT);
    bottom_rect.setColor(0, 0, 0);
    
    top_line_rect.setColor(100, 100, 100);
    top_line_rect.setHeight(LINE_HEIGHT);
    top_line_rect.setY(TOP_HEIGHT);
    
    bottom_line_rect.setColor(100, 100, 100);
    bottom_line_rect.setHeight(LINE_HEIGHT);
    bottom_line_rect.setY((short)-BOTTOM_HEIGHT);
    
    x_texture = swl::loadTextureFromFile("texturePack/x-button.png", &x_width, &x_height);
}

bool ends_with(const std::string& value, std::string ending) {
    return ending.size() <= value.size() && std::equal(ending.rbegin(), ending.rend(), value.rbegin());
}

void reload() {
    position = 0;
    scroll_limit = 0;
    
    worlds.clear();
    worlds_names.clear();

    std::vector<std::string> banned_dirs = {
            "..",
            ".",
            ".DS_Store",
    };

    DIR *dir = opendir(fileSystem::worlds_dir.c_str());
    dirent *ent;
    while((ent = readdir(dir)) != nullptr) {
        std::string name = ent->d_name;
        if(!std::count(banned_dirs.begin(), banned_dirs.end(), name) && ends_with(name, ".world"))
            worlds.emplace_back(name.substr(0, name.size()-6));
    }
    closedir (dir);
    
    for(auto& world : worlds) {
        world.button.setScale(3);
        world.button.setColor(0, 0, 0);
        world.button.setHoverColor(100, 100, 100);
        world.button.setText(world.name, 255, 255, 255);
        world.button_y = scroll_limit + title.getHeight() + 2 * PADDING;
        
        world.delete_button.setFreeTexture(false);
        world.delete_button.setTexture(x_texture, x_width, x_height);
        world.delete_button.setScale(3);
        world.delete_button.setColor(0, 0, 0);
        world.delete_button.setHoverColor(100, 100, 100);
        world.delete_button.x = short(world.button.getWidth() / 2 + world.delete_button.getWidth() / 2 + PADDING);
        
        scroll_limit += world.button.getHeight() + PADDING;
        
        worlds_names.push_back(world.name);
    }
}

void worldSelector::loop() {
    bool running = true, display_hover;
    SDL_Event event;
    
    reload();
    
    while(running && !gameLoop::quit) {
        framerateRegulator::regulateFramerate();
        while(SDL_PollEvent(&event)) {
            if(swl::handleBasicEvents(event, &running) && !running)
                gameLoop::quit = true;
            else if(back_button.isPressed(event))
                running = false;
            else if(new_button.isPressed(event)) {
                worldCreator::loop(worlds_names);
                reload();
            }
            else if(event.type == SDL_MOUSEWHEEL) {
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
                        gameLoop::main(world.name);
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
}
