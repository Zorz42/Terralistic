#include "menuBack.hpp"
#include "resourcePath.hpp"
#include "readOpa.hpp"

void MenuBack::init() {
    loadOpa(background, resource_path + "background.opa");
    
    back_rect.orientation = gfx::CENTER;
    back_rect.fill_color.a = TRANSPARENCY;
    back_rect.blur_radius = BLUR;
    back_rect.shadow_intensity = SHADOW_INTENSITY;
    back_rect.border_color = BORDER_COLOR;
    back_rect.smooth_factor = 3;
}

void MenuBack::renderBack() {
    float scale = (float)gfx::getWindowHeight() / (float)background.getTextureHeight();
    int pos = int(timer.getTimeElapsed() / 30) % int(background.getTextureWidth() * scale);
    background.render(scale, pos, 0);
    background.render(scale, pos - background.getTextureWidth() * scale, 0);
    
    back_rect.setHeight(gfx::getWindowHeight());
    back_rect.render();
}
