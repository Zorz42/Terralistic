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

#undef main

void startMenu::main() {
    bool running = true;
    SDL_Event event;
    
    ui::button play_button;
    play_button.setColor(0, 0, 0);
    play_button.setHoverColor(100, 100, 100);
    play_button.setScale(3);
    play_button.setText("Play", 255, 255, 255);
    play_button.y = -play_button.getHeight() / 2 - 1;
    
    ui::button exit_button;
    exit_button.setColor(0, 0, 0);
    exit_button.setHoverColor(100, 100, 100);
    exit_button.setScale(3);
    exit_button.setText("Exit", 255, 255, 255);
    exit_button.y = exit_button.getHeight() / 2;
    
    while(running && !gameLoop::quit) {
        framerateRegulator::regulateFramerate();
        while(SDL_PollEvent(&event)) {
            if(swl::handleBasicEvents(event, &running));
            else if(play_button.isPressed(event))
                worldSelector::loop();
            else if(exit_button.isPressed(event))
                running = false;
        }
        
        swl::setDrawColor(0, 0, 0);
        swl::clear();
        
        play_button.render();
        exit_button.render();
        
        swl::update();
    }
}
