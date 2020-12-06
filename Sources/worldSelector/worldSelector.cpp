//
//  worldSelector.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#include <filesystem>
#include <vector>

#include "worldSelector.hpp"
#include "singleWindowLibrary.hpp"
#include "UIKit.hpp"
#include "framerateRegulator.hpp"
#include "gameLoop.hpp"
#include "fileSystem.hpp"
#include "worldCreator.hpp"


struct world_to_select {
    std::string name;
    world_to_select(std::string name) : name(name) {}
    ui::button button{(ogl::objectType::top)};
    void render(bool display_hover);
    int button_y;
};

ogl::texture title(ogl::objectType::top);
ui::button back_button(ogl::objectType::bottom), new_button(ogl::objectType::bottom_right);
ogl::rect top_rect(ogl::objectType::top), bottom_rect(ogl::objectType::bottom), top_line_rect(ogl::objectType::top), bottom_line_rect(ogl::objectType::bottom);
int position;
std::vector<std::string> worlds_names;
std::vector<world_to_select> worlds;
int scroll_limit;

void world_to_select::render(bool display_hover) {
    button.setY(button_y - position);
    button.render(display_hover);
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
    back_button.setTextColor(255, 255, 255);
    back_button.setScale(3);
    back_button.setText("Back");
    back_button.setY(-PADDING);
    
    new_button.setColor(0, 0, 0);
    new_button.setHoverColor(100, 100, 100);
    new_button.setTextColor(255, 255, 255);
    new_button.setScale(3);
    new_button.setText("New");
    new_button.setY(-PADDING);
    new_button.setX(-PADDING);

    top_rect.setHeight(TOP_HEIGHT);
    top_rect.setColor(0, 0, 0);
    
    bottom_rect.setHeight(BOTTOM_HEIGHT);
    bottom_rect.setColor(0, 0, 0);
    
    top_line_rect.setColor(100, 100, 100);
    top_line_rect.setHeight(LINE_HEIGHT);
    top_line_rect.setY(TOP_HEIGHT);
    
    bottom_line_rect.setColor(100, 100, 100);
    bottom_line_rect.setHeight(LINE_HEIGHT);
    bottom_line_rect.setY(-BOTTOM_HEIGHT);
}

void reload() {
    position = 0;
    scroll_limit = 0;
    
    worlds.clear();
    worlds_names.clear();
    
    for(const auto& world : std::filesystem::directory_iterator(fileSystem::worlds_dir))
        if(world.path().filename().string() != ".DS_Store")
            worlds.push_back(world.path().filename().string());
    
    for(int i = 0; i < worlds.size(); i++) {
        world_to_select& world = worlds.at(i);
        world.button.setScale(3);
        world.button.setColor(0, 0, 0);
        world.button.setHoverColor(100, 100, 100);
        world.button.setTextColor(255, 255, 255);
        world.button.setText(world.name);
        world.button_y = scroll_limit + title.getHeight() + 2 * PADDING;
        scroll_limit += world.button.getHeight() + PADDING;
        
        worlds_names.push_back(world.name);
    }
}

void worldSelector::loop() {
    bool running = true, display_hover = true;
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
