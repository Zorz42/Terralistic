#include "settings.hpp"

void Settings::init() {
    back_button.renderText("Back");
    back_button.scale = 3;
    back_button.orientation = gfx::TOP;
}

void Settings::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT) {
        if(back_button.isHovered(mouse_x, mouse_y))
            gfx::returnFromScene();
    }
}

void Settings::render() {
    background->setBackWidth(400);
    background->renderBack();
    
    back_button.render(mouse_x, mouse_y);
}
