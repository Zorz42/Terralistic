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
    int pos = int(timer.getTimeElapsed() * 1000 / 30 / gfx::getWindowWidth()) % int(background.getTextureWidth() * scale);
    
    for(int i = -1; i < gfx::getWindowWidth() / (background.getTextureWidth() * scale) + 2; i++)
        background.render(scale, pos + i * background.getTextureWidth() * scale, 0);
    
    if(back_rect.getHeight() != gfx::getWindowHeight()) {
        back_rect.setHeight(gfx::getWindowHeight());
        back_rect.jumpToTarget();
    }
    back_rect.render();
}
