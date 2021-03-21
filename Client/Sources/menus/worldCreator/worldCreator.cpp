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
#include "gameLoop.hpp"
#include "main.hpp"
#include "init.hpp"

// menu, where you create worlds
// for guide, check multiplayer selector, which is quite similar

static gfx::button back_button, create_button;
static gfx::sprite new_world_title, faded_create;
static gfx::textInput world_name_input;

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

    world_name_input.scale = 3;
    world_name_input.orientation = gfx::center;
INIT_SCRIPT_END

/*void renderTextCreator() {
    if(!name.empty()) {
        world_name.loadFromText(name, {255, 255, 255});
        world_name.setY(short(-world_name.getHeight() / 7));
    }
}

void worldCreator::loop(std::vector<std::string> worlds) {
    bool running = true, can_create;
    SDL_Event event;
    name = "";
    
    renderTextCreator();
    
    while(running && main_::running) {
        can_create = !name.empty() && !std::count(worlds.begin(), worlds.end(), name);
        
        while(SDL_PollEvent(&event)) {
            SDL_StartTextInput();
            if(swl::handleBasicEvents(event, &main_::running));
            else if(can_create && (create_button.isPressed(event) || (event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_RETURN))) {
                gameLoop::main(name, false);
                running = false;
            }
            else if(back_button_creator.isPressed(event))
                running = false;
            else if(event.type == SDL_TEXTINPUT) {
                char c = event.text.text[0];
                if(c == ' ')
                    c = '-';
                if((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '-') {
                    name.push_back(c);
                    renderTextCreator();
                }
            }
            else if(event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_BACKSPACE && !name.empty()) {
                name.pop_back();
                renderTextCreator();
            }
        }
        
        swl::setDrawColor(0, 0, 0);
        swl::clear();
        
        if(can_create)
            create_button.render();
        else
            faded_create.render();
        back_button_creator.render();
        new_world_title.render();
        if(!name.empty())
            world_name.render();
        
        swl::update();
    }
}*/

void worldCreator::scene::init() {
    world_name_input.setText("");
    text_inputs = {&world_name_input};
}

void worldCreator::scene::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_MOUSE_LEFT) {
        if(back_button.isHovered())
            gfx::returnFromScene();
        else if(create_button.isHovered())
            gfx::switchScene(new gameLoop::scene(world_name_input.getText(), false));
    }
}

void worldCreator::scene::render() {
    can_create = !world_name_input.getText().empty() && !std::count(worlds.begin(), worlds.end(), world_name_input.getText());
    if(can_create)
        gfx::render(create_button);
    else
        gfx::render(faded_create);
    gfx::render(back_button);
    gfx::render(new_world_title);
    gfx::render(world_name_input);
}
