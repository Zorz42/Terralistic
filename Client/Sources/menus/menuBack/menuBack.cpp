#include "menuBack.hpp"

void MenuBack::init() {
    background.loadFromFile("background.png");
    
    back_rect.orientation = gfx::CENTER;
    back_rect.w = 0;
    back_rect.c.a = TRANSPARENCY;
    back_rect.blur_intensity = BLUR;
    back_rect.enableShadow();
}

void MenuBack::render() {
    float scale = (float)gfx::getWindowHeight() / (float)background.getTextureHeight();
    int pos = gfx::getTicks() / 30 % int(background.getTextureWidth() * scale);
    background.render(scale, pos, 0);
    background.render(scale, pos - background.getTextureWidth() * scale, 0);
    
    back_rect.w += (width - back_rect.w) / 2;
    back_rect.h = gfx::getWindowHeight();
    back_rect.render();
}
