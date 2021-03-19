//
//  multiplayerSelector.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#include "core.hpp"

#include <vector>
#include <algorithm>
#include "multiplayerSelector.hpp"
#include "UIKit.hpp"
#include "singleWindowLibrary.hpp"
#include "gameLoop.hpp"
#include "main.hpp"
#include "init.hpp"

// this is a menu, where you select the server you want to play on

static ui::button back_button_multiplayer(ogl::bottom), join_button(ogl::bottom);
static ogl::texture join_server_title(ogl::top), server_ip;
static std::string ip;

#define PADDING 20

INIT_SCRIPT
    // set all graphical elements
    
    // the back button
    /*back_button_multiplayer.setColor(0, 0, 0);
    back_button_multiplayer.setHoverColor(100, 100, 100);
    back_button_multiplayer.setScale(3);
    back_button_multiplayer.setText("Back", 255, 255, 255);
    back_button_multiplayer.y = -PADDING;
    
    // "Type the server IP:" button in the top
    join_server_title.loadFromText("Type the server IP:", {255, 255, 255});
    join_server_title.scale = 3;
    join_server_title.setY(PADDING);
    
    // "Join Server" button
    join_button.setColor(0, 0, 0);
    join_button.setHoverColor(100, 100, 100);
    join_button.setScale(3);
    join_button.setText("Join Server", 255, 255, 255);
    join_button.y = -PADDING;
    
    back_button_multiplayer.x = short((-join_button.getWidth() - back_button_multiplayer.getWidth() + back_button_multiplayer.getWidth() - PADDING) / 2);
    join_button.x = short((join_button.getWidth() + back_button_multiplayer.getWidth() - join_button.getWidth() + PADDING) / 2);
    
    server_ip.scale = 3;*/
INIT_SCRIPT_END

void renderTextMultiplayer() {
    // update ip texture if it changed
    if(!ip.empty()) {
        server_ip.loadFromText(ip, {255, 255, 255});
        server_ip.setY(short(-server_ip.getHeight() / 7));
    }
}

void multiplayerSelector::loop() {
    bool running = true;
    SDL_Event event;
    ip = "";
    
    renderTextMultiplayer();
    
    while(running && main_::running) {
        while(SDL_PollEvent(&event)) {
            SDL_StartTextInput();
            if(swl::handleBasicEvents(event, &main_::running));
            else if(join_button.isPressed(event) || (event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_RETURN)) { // join button or enter is pressed
                gameLoop::main(ip, true);
                running = false;
            }
            else if(back_button_multiplayer.isPressed(event)) // escape from loop if back button is pressed
                running = false;
            else if(event.type == SDL_TEXTINPUT) { // detect input from keyboard
                char c = event.text.text[0];
                if(c == ' ')
                    c = '-';
                if((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '-' || c == '.') { // filter out characters
                    ip.push_back(c);
                    renderTextMultiplayer();
                }
            }
            else if(event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_BACKSPACE && !ip.empty()) { // backspace - deleate a character
                ip.pop_back();
                renderTextMultiplayer();
            }
        }
        
        swl::setDrawColor(0, 0, 0);
        swl::clear();
        
        join_button.render();
        back_button_multiplayer.render();
        join_server_title.render();
        if(!ip.empty())
            server_ip.render();
        
        swl::update();
    }
}
