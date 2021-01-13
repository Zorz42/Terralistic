//
//  startMenu.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/07/2020.
//

#include "startMenu.hpp"
#include "singleWindowLibrary.hpp"
#include "UIKit.hpp"
#include "framerateRegulator.hpp"
#include "gameLoop.hpp"
#include "worldSelector.hpp"
#include "multiplayerSelector.hpp"

#undef main

void startMenu::main() {
    bool running = true;
    SDL_Event event;

    ui::button singleplayer_button;
    singleplayer_button.setColor(0, 0, 0);
    singleplayer_button.setHoverColor(100, 100, 100);
    singleplayer_button.setScale(3);
    singleplayer_button.setText("Singleplayer", 255, 255, 255);
    singleplayer_button.y = short(-singleplayer_button.getHeight() - 5);
    
    ui::button multiplayer_button;
    multiplayer_button.setColor(0, 0, 0);
    multiplayer_button.setHoverColor(100, 100, 100);
    multiplayer_button.setScale(3);
    multiplayer_button.setText("Multiplayer", 255, 255, 255);
    
    ui::button exit_button;
    exit_button.setColor(0, 0, 0);
    exit_button.setHoverColor(100, 100, 100);
    exit_button.setScale(3);
    exit_button.setText("Exit", 255, 255, 255);
    exit_button.y = short(exit_button.getHeight() + 5);
    
    while(running && !gameLoop::quit) {
        framerateRegulator::regulateFramerate();
        while(SDL_PollEvent(&event)) {
            if(swl::handleBasicEvents(event, &running));
            else if(singleplayer_button.isPressed(event))
                worldSelector::loop();
            else if(multiplayer_button.isPressed(event))
                multiplayerSelector::loop();
            else if(exit_button.isPressed(event))
                running = false;
        }
        
        swl::setDrawColor(0, 0, 0);
        swl::clear();
        
        singleplayer_button.render();
        multiplayer_button.render();
        exit_button.render();
        
        swl::update();
    }
}
