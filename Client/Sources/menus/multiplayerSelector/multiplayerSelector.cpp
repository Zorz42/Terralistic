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
#include "game.hpp"
#include "main.hpp"
#include "init.hpp"

// this is a menu, where you select the server you want to play on

#define PADDING 20

void multiplayerSelector::scene::init() {
    // the back button
    back_button.scale = 3;
    back_button.setTexture(gfx::renderText("Back", {255, 255, 255}));
    back_button.y = -PADDING;
    back_button.orientation = gfx::bottom;
    
    // "Type the server IP:" button in the top
    join_server_title.setTexture(gfx::renderText("Type the server IP:", {255, 255, 255}));
    join_server_title.scale = 3;
    join_server_title.y = PADDING;
    join_server_title.orientation = gfx::top;
    
    // "Join Server" button
    join_button.scale = 3;
    join_button.setTexture(gfx::renderText("Join Server", {255, 255, 255}));
    join_button.y = -PADDING;
    join_button.orientation = gfx::bottom;
    
    back_button.x = short((-join_button.getWidth() - back_button.getWidth() + back_button.getWidth() - PADDING) / 2);
    join_button.x = short((join_button.getWidth() + back_button.getWidth() - join_button.getWidth() + PADDING) / 2);
    
    server_ip.scale = 3;
    server_ip.orientation = gfx::center;
    server_ip.setText("");
    server_ip.active = true;
    
    one_time = true;
    text_inputs = {&server_ip};
}

void multiplayerSelector::scene::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_MOUSE_LEFT && back_button.isHovered())
        gfx::returnFromScene();
    else if((key == gfx::KEY_MOUSE_LEFT && join_button.isHovered()) || key == gfx::KEY_ENTER)
        gfx::switchScene(new game::scene(server_ip.getText(), true));
}

void multiplayerSelector::scene::render() {
    gfx::render(join_button);
    gfx::render(back_button);
    gfx::render(join_server_title);
    gfx::render(server_ip);
}
