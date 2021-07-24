//
//  worldCreator.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#include <algorithm>
#include "worldCreator.hpp"
#include "game.hpp"

// menu, where you create worlds
// for guide, check multiplayer selector, which is quite similar

#define PADDING 20

void worldCreator::init() {
    back_button.scale = 3;
    back_button.renderText("Back", {255, 255, 255});
    back_button.y = -PADDING;
    back_button.orientation = gfx::BOTTOM;
    
    new_world_title.renderText("New world name:", {255, 255, 255});
    new_world_title.scale = 3;
    new_world_title.y = PADDING;
    new_world_title.orientation = gfx::TOP;
    
    create_button.scale = 3;
    create_button.renderText("Create world", {255, 255, 255});
    create_button.y = -PADDING;
    create_button.orientation = gfx::BOTTOM;

    back_button.x = (-create_button.getWidth() - back_button.getWidth() + back_button.getWidth() - PADDING) / 2;
    create_button.x = (create_button.getWidth() + back_button.getWidth() - create_button.getWidth() + PADDING) / 2;

    world_name.scale = 3;
    world_name.orientation = gfx::CENTER;
    world_name.setText("");
    world_name.active = true;
    world_name.textProcessing = [](char c, int length) {
        if((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '-' || c == '_')
            return c;
        if(c == ' ')
            return '-';
        return '\0';
    };
    
    text_inputs = {&world_name};
}

void worldCreator::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT && back_button.isHovered())
        gfx::returnFromScene();
    else if((key == gfx::Key::MOUSE_LEFT && create_button.isHovered()) || (key == gfx::Key::ENTER && can_create)) {
        startPrivateWorld(world_name.getText());
        gfx::returnFromScene();
    }
}

void worldCreator::render() {
    if(can_create != (!world_name.getText().empty() && !std::count(worlds.begin(), worlds.end(), world_name.getText()))) {
        can_create = !can_create;
        create_button.renderText("Create world", {(unsigned char)(can_create ? 255 : 100), (unsigned char)(can_create ? 255 : 100), (unsigned char)(can_create ? 255 : 100)});
        create_button.disabled = !can_create;
    }
    create_button.render();
    back_button.render();
    new_world_title.render();
    world_name.render();
}
