#include <cmath>
#include "rect.hpp"
#include "glfwAbstraction.hpp"
#include "exception.hpp"
#include "blur.hpp"
#include "shadow.hpp"

float approach(float object, int target, int smooth_factor) {
    return object + (target - object) / smooth_factor;
}

void gfx::Rect::render() {
    first_time = false;
    
    while(ms_counter < approach_timer.getTimeElapsed()) {
        ms_counter++;
        x = approach(x, target_x, smooth_factor * 10);
        y = approach(y, target_y, smooth_factor * 10);
        width = approach(width, target_width, smooth_factor * 10);
        height = approach(height, target_height, smooth_factor * 10);
    }
    
    RectShape rect = getTranslatedRect();

    if(blur_radius && blur_enabled)
        gfx::blurRectangle(rect, blur_radius, window_texture, window_texture_back, getWindowWidth(), getWindowHeight(), normalization_transform);
    
    if(shadow_intensity) {
        float shadow_edge_width = std::min(200.f + (float)width / 2, 350.f), shadow_edge_height = std::min(200.f + (float)height / 2, 350.f);
        
        shadow_texture->render(1, rect.x - 200, rect.y - 200, {0, 0, (int)std::floor(shadow_edge_width), 200}, false, {255, 255, 255, shadow_intensity});
        shadow_texture->render(1, rect.x - 200, rect.y, {0, 200, 200, (int)std::ceil(shadow_edge_height) - 200}, false, {255, 255, 255, shadow_intensity});

        shadow_texture->render(1, rect.x + width - std::ceil(shadow_edge_width) + 200, rect.y - 200, {700 - (int)std::ceil(shadow_edge_width), 0, (int)std::ceil(shadow_edge_width), 200}, false, {255, 255, 255, shadow_intensity});
        shadow_texture->render(1, rect.x + width, rect.y, {500, 200, 200, (int)std::ceil(shadow_edge_height) - 200}, false, {255, 255, 255, shadow_intensity});

        shadow_texture->render(1, rect.x - 200, rect.y + height - std::floor(shadow_edge_height) + 200, {0, 700 - (int)std::floor(shadow_edge_height), 200, (int)std::floor(shadow_edge_height) - 200}, false, {255, 255, 255, shadow_intensity});
        shadow_texture->render(1, rect.x - 200, rect.y + height, {0, 500, (int)std::floor(shadow_edge_width), 200}, false, {255, 255, 255, shadow_intensity});

        shadow_texture->render(1, rect.x + width, rect.y + height - std::floor(shadow_edge_height) + 200, {500, 700 - (int)std::floor(shadow_edge_height), 200, (int)std::floor(shadow_edge_height) - 200}, false, {255, 255, 255, shadow_intensity});
        shadow_texture->render(1, rect.x + width - std::ceil(shadow_edge_width) + 200, rect.y + height, {700 - (int)std::ceil(shadow_edge_width), 500, (int)std::ceil(shadow_edge_width), 200}, false, {255, 255, 255, shadow_intensity});
        
        if(shadow_edge_height == 350) {
            int height_to_render = height - 300;
            while(height_to_render > 0) {
                shadow_texture->render(1, rect.x - 200, rect.y + height - 150 - height_to_render, {0, 300, 200, std::min(100, height_to_render)}, false, {255, 255, 255, shadow_intensity});
                shadow_texture->render(1, rect.x + width, rect.y + height - 150 - height_to_render, {500, 300, 200, std::min(100, height_to_render)}, false, {255, 255, 255, shadow_intensity});
                height_to_render -= 100;
            }
        }
        
        if(shadow_edge_width == 350) {
            int width_to_render = width - 300;
            while(width_to_render > 0) {
                shadow_texture->render(1, rect.x + width - 150 - width_to_render, rect.y - 200, {300, 0, std::min(100, width_to_render), 200}, false, {255, 255, 255, shadow_intensity});
                shadow_texture->render(1, rect.x + width - 150 - width_to_render, rect.y + height, {300, 500, std::min(100, width_to_render), 200}, false, {255, 255, 255, shadow_intensity});
                width_to_render -= 100;
            }
        }

    }
    
    rect.render(fill_color);
    rect.renderOutline(border_color);
}

int gfx::Rect::getWidth() const {
    return width;
}

void gfx::Rect::setWidth(int width_) {
    if(width_ < 0)
        throw Exception("Width must be positive.");
    target_width = width_;
    if(first_time)
        width = width_;
}

int gfx::Rect::getHeight() const {
    return height;
}

void gfx::Rect::setHeight(int height_) {
    if(height_ < 0)
        throw Exception("Height must be positive.");
    target_height = height_;
    if(first_time)
        height = height_;
}

int gfx::Rect::getX() const {
    return x;
}

void gfx::Rect::setX(int x_) {
    target_x = x_;
    if(first_time)
        x = x_;
}

int gfx::Rect::getY() const {
    return y;
}

void gfx::Rect::setY(int y_) {
    target_y = y_;
    if(first_time)
        y = y_;
}

int gfx::Rect::getTargetX() const {
    return target_x;
}

int gfx::Rect::getTargetY() const {
    return target_y;
}

void gfx::Rect::jumpToTarget() {
    y = target_y;
    x = target_x;
    width = target_width;
    height = target_height;
}
