//
//  debugMenu.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/06/2021.
//

#include "debugMenu.hpp"

void debugMenu::init() {
    fps_text.scale = 3;
    fps_text.x = 10;
    fps_text.y = 10;
    fps_text.orientation = gfx::top_left;
    
    renderFpsText();
    
    coords_text.scale = 3;
    coords_text.y = fps_text.y + fps_text.getHeight() + 10;
    coords_text.x = 10;
    coords_text.orientation = gfx::top_left;
}

void debugMenu::update() {
    static unsigned int count = gfx::getTicks() / 1000 - 1;
    fps_count++;
    if(gfx::getTicks() / 1000 > count) {
        count++;
        if(debug_menu_open)
            renderFpsText();
        fps_count = 0;
    }
    
    if(debug_menu_open) {
        static unsigned int prev_x = 0, prev_y = 0;
        unsigned int curr_x = main_player->position_x / BLOCK_WIDTH, curr_y = main_player->position_y / BLOCK_WIDTH;
        if(curr_x != prev_x || curr_y != prev_y) {
            prev_x = curr_x;
            prev_y = curr_y;
            coords_text.setTexture(gfx::renderText(std::string("X: ") + std::to_string(main_player->position_x / BLOCK_WIDTH) + ", Y: " + std::to_string(main_player->position_y / BLOCK_WIDTH), {0, 0, 0}));
        }
    }
}

void debugMenu::render() {
    if(debug_menu_open) {
        gfx::render(fps_text);
        gfx::render(coords_text);
    }
}

void debugMenu::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_M && !m_down) {
        m_down = true;
        debug_menu_open = !debug_menu_open;
        if(debug_menu_open)
            renderFpsText();
    }
}

void debugMenu::onKeyUp(gfx::key key) {
    if(key == gfx::KEY_M)
        m_down = false;
}

void debugMenu::renderFpsText() {
    fps_text.setTexture(gfx::renderText(std::to_string(fps_count) + " fps", {0, 0, 0}));
}
