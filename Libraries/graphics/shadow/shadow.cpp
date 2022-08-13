#include <cmath>
#include "theme.hpp"
#include "shadow.hpp"
#include "blur.hpp"
#include "glfwAbstraction.hpp"

void gfx::initShadow() {
    Surface surface;
    surface.createEmpty(700, 700);
    
    Texture shadow_texture_back;
    shadow_texture_back.loadFromSurface(surface);
    
    for(int y = 200; y < 500; y++)
        for(int x = 200; x < 500; x++)
            surface.setPixel(x, y, {0, 0, 0});
    shadow_texture = new Texture;
    shadow_texture->loadFromSurface(surface);
    
    glFramebufferTexture(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, shadow_texture->_getGlTexture(), 0);
    glViewport(0, 0, 700, 700);
    
    _Transformation shadow_transform = shadow_texture->_getNormalizationTransform();
    shadow_transform.translate(-700, 0);
    shadow_transform.stretch(2, 2);
    for(int i = 0; i < 100; i++)
        blurRectangle(RectShape(0, 0, 700, 700), GFX_SHADOW_BLUR, shadow_texture->_getGlTexture(), shadow_texture_back._getGlTexture(), 700, 700, shadow_transform);
    
    glFramebufferTexture(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, window_texture, 0);
    glViewport(0, 0, getWindowWidth(), getWindowHeight());
}

void gfx::quitShadow() {
    delete shadow_texture;
}

void gfx::drawShadow(RectShape rect, unsigned char shadow_intensity) {
    float shadow_edge_width = std::min(200.f + rect.w / 2.f, 350.f), shadow_edge_height = std::min(200.f + rect.h / 2.f, 350.f);
    
    shadow_texture->render(1, rect.x - 200, rect.y - 200, {0, 0, (int)std::floor(shadow_edge_width), 200}, false, {255, 255, 255, shadow_intensity});
    shadow_texture->render(1, rect.x - 200, rect.y, {0, 200, 200, (int)std::ceil(shadow_edge_height) - 200}, false, {255, 255, 255, shadow_intensity});

    shadow_texture->render(1, rect.x + rect.w - std::ceil(shadow_edge_width) + 200, rect.y - 200, {700 - (int)std::ceil(shadow_edge_width), 0, (int)std::ceil(shadow_edge_width), 200}, false, {255, 255, 255, shadow_intensity});
    shadow_texture->render(1, rect.x + rect.w, rect.y, {500, 200, 200, (int)std::ceil(shadow_edge_height) - 200}, false, {255, 255, 255, shadow_intensity});

    shadow_texture->render(1, rect.x - 200, rect.y + rect.h - std::floor(shadow_edge_height) + 200, {0, 700 - (int)std::floor(shadow_edge_height), 200, (int)std::floor(shadow_edge_height) - 200}, false, {255, 255, 255, shadow_intensity});
    shadow_texture->render(1, rect.x - 200, rect.y + rect.h, {0, 500, (int)std::floor(shadow_edge_width), 200}, false, {255, 255, 255, shadow_intensity});

    shadow_texture->render(1, rect.x + rect.w, rect.y + rect.h - std::floor(shadow_edge_height) + 200, {500, 700 - (int)std::floor(shadow_edge_height), 200, (int)std::floor(shadow_edge_height) - 200}, false, {255, 255, 255, shadow_intensity});
    shadow_texture->render(1, rect.x + rect.w - std::ceil(shadow_edge_width) + 200, rect.y + rect.h, {700 - (int)std::ceil(shadow_edge_width), 500, (int)std::ceil(shadow_edge_width), 200}, false, {255, 255, 255, shadow_intensity});
    
    if(shadow_edge_height == 350) {
        int height_to_render = rect.h - 300;
        while(height_to_render > 0) {
            shadow_texture->render(1, rect.x - 200, rect.y + rect.h - 150 - height_to_render, {0, 300, 200, std::min(100, height_to_render)}, false, {255, 255, 255, shadow_intensity});
            shadow_texture->render(1, rect.x + rect.w, rect.y + rect.h - 150 - height_to_render, {500, 300, 200, std::min(100, height_to_render)}, false, {255, 255, 255, shadow_intensity});
            height_to_render -= 100;
        }
    }
    
    if(shadow_edge_width == 350) {
        int width_to_render = rect.w - 300;
        while(width_to_render > 0) {
            shadow_texture->render(1, rect.x + rect.w - 150 - width_to_render, rect.y - 200, {300, 0, std::min(100, width_to_render), 200}, false, {255, 255, 255, shadow_intensity});
            shadow_texture->render(1, rect.x + rect.w - 150 - width_to_render, rect.y + rect.h, {300, 500, std::min(100, width_to_render), 200}, false, {255, 255, 255, shadow_intensity});
            width_to_render -= 100;
        }
    }
}
