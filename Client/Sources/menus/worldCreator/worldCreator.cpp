//
//  worldCreator.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#include "core.hpp"

#include <vector>
#include <algorithm>
#include "worldCreator.hpp"
#include "game.hpp"
#include "main.hpp"
#include "init.hpp"

// menu, where you create worlds
// for guide, check multiplayer selector, which is quite similar

static gfx::button back_button, create_button;
static gfx::sprite new_world_title, faded_create;
static gfx::textInput world_name;

#define PADDING 20

INIT_SCRIPT
    back_button.scale = 3;
    back_button.setTexture(gfx::renderText("Back", {255, 255, 255}));
    back_button.y = -PADDING;
    back_button.orientation = gfx::bottom;
    
    new_world_title.setTexture(gfx::renderText("New world name:", {255, 255, 255}));
    new_world_title.scale = 3;
    new_world_title.y = PADDING;
    new_world_title.orientation = gfx::top;
    
    create_button.scale = 3;
    create_button.setTexture(gfx::renderText("Create world", {255, 255, 255}));
    create_button.y = -PADDING;
    create_button.orientation = gfx::bottom;

    back_button.x = (-create_button.getWidth() - back_button.getWidth() + back_button.getWidth() - PADDING) / 2;
    create_button.x = (create_button.getWidth() + back_button.getWidth() - create_button.getWidth() + PADDING) / 2;
    
    faded_create.setTexture(gfx::renderText("Create world", {100, 100, 100}));
    faded_create.scale = 3;
    faded_create.x = (create_button.getWidth() + back_button.getWidth() - create_button.getWidth() + PADDING) / 2;
    faded_create.y = -PADDING - 10 * faded_create.scale;
    faded_create.orientation = gfx::bottom;

    world_name.scale = 3;
    world_name.orientation = gfx::center;
INIT_SCRIPT_END

void worldCreator::scene::init() {
    world_name.setText("");
    world_name.active = true;
    one_time = true;
    text_inputs = {&world_name};
}

void worldCreator::scene::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_MOUSE_LEFT && back_button.isHovered())
        gfx::returnFromScene();
    else if((key == gfx::KEY_MOUSE_LEFT && create_button.isHovered()) || key == gfx::KEY_ENTER)
        gfx::switchScene(new game::scene(world_name.getText(), false));
}

void worldCreator::scene::render() {
    can_create = !world_name.getText().empty() && !std::count(worlds.begin(), worlds.end(), world_name.getText());
    if(can_create)
        gfx::render(create_button);
    else
        gfx::render(faded_create);
    gfx::render(back_button);
    gfx::render(new_world_title);
    gfx::render(world_name);
}
