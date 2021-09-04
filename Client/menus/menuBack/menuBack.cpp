#include "menuBack.hpp"

void MenuBack::init() {
    background.loadFromResources("background.png");
    
    back_rect.orientation = gfx::CENTER;
    back_rect.fill_color.a = TRANSPARENCY;
    back_rect.blur_intensity = BLUR;
    back_rect.shadow_intensity = SHADOW_INTENSITY;
    back_rect.border_color = BORDER_COLOR;
    back_rect.smooth_factor = 3;
}

void MenuBack::renderBack() {
    float scale = (float)gfx::getWindowHeight() / (float)background.getTextureHeight();
    int pos = gfx::getTicks() / 30 % int(background.getTextureWidth() * scale);
    background.render(scale, pos, 0);
    background.render(scale, pos - background.getTextureWidth() * scale, 0);
    
    back_rect.setHeight(gfx::getWindowHeight());
    back_rect.render();
}
