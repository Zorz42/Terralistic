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
    fps_text.orientation = gfx::TOP_LEFT;
    
    renderFpsText();
    
    coords_text.scale = 3;
    coords_text.y = fps_text.y + fps_text.getHeight() + 10;
    coords_text.x = 10;
    coords_text.orientation = gfx::TOP_LEFT;

    /*
    biome_text.scale = 3;
    biome_text.y = coords_text.y + coords_text.getHeight() + 10;
    biome_text.x = 10;
    biome_text.orientation = gfx::top_left;
    */
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
            coords_text.renderText(std::string("X: ") + std::to_string(main_player->position_x / BLOCK_WIDTH) + ", Y: " + std::to_string(world_map->getWorldHeight() - main_player->position_y / BLOCK_WIDTH), {0, 0, 0});
            //biome_text.renderText(std::to_string(world_map->biomes[0]), { 0, 0, 0 }));
        }
    }
}

void debugMenu::render() {
    if(debug_menu_open) {
        fps_text.render();
        coords_text.render();
    }
}

void debugMenu::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::M && !m_down) {
        m_down = true;
        debug_menu_open = !debug_menu_open;
        if(debug_menu_open)
            renderFpsText();
    }
}

void debugMenu::onKeyUp(gfx::Key key) {
    if(key == gfx::Key::M)
        m_down = false;
}

void debugMenu::renderFpsText() {
    fps_text.renderText(std::to_string(fps_count) + " fps", {0, 0, 0});
}
