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

struct world_to_select {
    std::string name;
    world_to_select(std::string name) : name(name) {}
    ui::button button{(ogl::objectType::top)};
    void render();
};

void world_to_select::render() {
    button.render();
}

void worldSelector::init() {
    
}

#define PADDING 20

void worldSelector::loop() {
    bool running = true;
    SDL_Event event;
    
    ogl::texture title(ogl::objectType::top);
    title.scale = 3;
    title.loadFromText("Select a world to play!", {255, 255, 255});
    title.setY(PADDING);
    
    ui::button back_button(ogl::objectType::bottom);
    back_button.setColor(0, 0, 0);
    back_button.setHoverColor(100, 100, 100);
    back_button.setTextColor(255, 255, 255);
    back_button.setScale(3);
    back_button.setText("Back");
    back_button.setY(-PADDING);
    
    std::vector<world_to_select> worlds;
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
        world.button.setY(i * world.button.getHeight() + (i + 2) * PADDING + title.getHeight());
    }
    
    
    
    while(running && !gameLoop::quit) {
        framerateRegulator::regulateFramerate();
        while(SDL_PollEvent(&event)) {
            if(swl::handleBasicEvents(event, &running) && !running)
                gameLoop::quit = true;
            else if(back_button.isPressed(event))
                running = false;
            else
                for(world_to_select& world : worlds)
                    if(world.button.isPressed(event)) {
                        gameLoop::main(world.name);
                        break;
                    }
        }
        
        swl::setDrawColor(0, 0, 0);
        swl::clear();
        
        for(world_to_select& world : worlds)
            world.render();
        
        title.render();
        back_button.render();
        
        swl::update();
    }
}
