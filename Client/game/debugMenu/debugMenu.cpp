#include "debugMenu.hpp"

void DebugMenu::init() {
    fps_text.scale = 3;
    fps_text.x = 10;
    fps_text.y = 10;
    fps_text.orientation = gfx::TOP_LEFT;
    updateFpsText();
    
    coords_text.scale = 3;
    coords_text.y = fps_text.y + fps_text.getHeight() + 10;
    coords_text.x = 10;
    coords_text.orientation = gfx::TOP_LEFT;
}

void DebugMenu::update() {
    static unsigned int count = gfx::getTicks() / 1000 - 1;
    fps_count++;
    if(gfx::getTicks() / 1000 > count) {
        count++;
        if(debug_menu_open)
            updateFpsText();
        fps_count = 0;
    }
    
    if(debug_menu_open) {
        static unsigned int prev_x = 0, prev_y = 0;
        unsigned int curr_x = player_handler->getMainPlayer()->getX() / (BLOCK_WIDTH * 2), curr_y = player_handler->getMainPlayer()->getY() / (BLOCK_WIDTH * 2);
        if(curr_x != prev_x || curr_y != prev_y) {
            prev_x = curr_x;
            prev_y = curr_y;
            updateCoordsText();
        }
    }
}

void DebugMenu::render() {
    if(debug_menu_open) {
        fps_text.render();
        coords_text.render();
    }
}

bool DebugMenu::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::M) {
        debug_menu_open = !debug_menu_open;
        if(debug_menu_open)
            updateFpsText();
        return true;
    }
    return false;
}

void DebugMenu::updateFpsText() {
    fps_text.loadFromText(std::to_string(fps_count) + " fps", BLACK);
}

void DebugMenu::updateCoordsText() {
    coords_text.loadFromText(std::string("X: ") + std::to_string(int(player_handler->getMainPlayer()->getX() / (BLOCK_WIDTH * 2))) + ", Y: " + std::to_string(int(blocks->getHeight() - player_handler->getMainPlayer()->getY() / (BLOCK_WIDTH * 2))), BLACK);
}
