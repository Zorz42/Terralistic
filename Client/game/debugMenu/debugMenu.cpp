#include "debugMenu.hpp"
#include "iostream"

void DebugMenu::init() {
    back_rect.orientation = gfx::BOTTOM_RIGHT;
    back_rect.setY(-SPACING);
    back_rect.fill_color = BLACK;
    back_rect.border_color = BORDER_COLOR;
    back_rect.fill_color.a = TRANSPARENCY;
    back_rect.blur_radius = BLUR;
    back_rect.smooth_factor = 3;
}

void DebugMenu::update(float frame_length) {
    if(debug_menu_open)
        for(DebugLine* debug_line : debug_lines)
            debug_line->update();
}

void DebugLine::render(int x, int y) {
    texture.render(2, x, y);
}

int DebugLine::getWidth() {
    return texture.getTextureWidth() * 2;
}

int DebugLine::getHeight() {
    return texture.getTextureHeight() * 2;
}

void DebugLine::update() {
    if(prev_text != text) {
        prev_text = text;
        texture.loadFromText(text);
    }
}

void DebugMenu::render() {
    int back_width = 0, back_height = 0;
    
    for(auto & debug_line : debug_lines) {
        back_width = std::max(debug_line->getWidth(), back_width);
        back_height += debug_line->getHeight();
    }
    
    back_rect.setWidth(back_width + SPACING);
    back_rect.setHeight(back_height + SPACING);
    back_rect.setX(debug_menu_open ? -SPACING : back_rect.getWidth() + SPACING);
    back_rect.render();
    
    int curr_y = gfx::getWindowHeight() + back_rect.getY() - back_rect.getHeight();
    for(auto & debug_line : debug_lines) {
        debug_line->render(gfx::getWindowWidth() + back_rect.getX() - back_rect.getWidth() + SPACING / 2, curr_y + SPACING / 2);
        curr_y += debug_line->getHeight();
    }
}


bool DebugMenu::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::M) {
        if(!debug_menu_open) // I know its ugly but if I do debug_menu_open = !debug_menu_open; for some reason clang-tidy thinks the variable not changing, and starts recommending weird optimizations.
            debug_menu_open = true;
        else
            debug_menu_open = false;
        return true;
    }
    return false;
}

void DebugMenu::registerDebugLine(DebugLine* debug_line) {
    debug_lines.push_back(debug_line);
}
