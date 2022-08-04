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
        
        if(std::abs(x - target_x) < 1)
            x = target_x;
        if(x != target_x)
            x = approach(x, target_x, smooth_factor * 10);
        
        if(std::abs(y - target_y) < 1)
            y = target_y;
        if(y != target_y)
            y = approach(y, target_y, smooth_factor * 10);
        
        if(std::abs(width - target_width) < 1)
            width = target_width;
        if(width != target_width)
            width = approach(width, target_width, smooth_factor * 10);
        
        if(std::abs(height - target_height) < 1)
            height = target_height;
        if(height != target_height)
            height = approach(height, target_height, smooth_factor * 10);
    }
    
    RectShape rect = getTranslatedRect();

    if(blur_radius && blur_enabled)
        gfx::blurRectangle(rect, blur_radius, window_texture, window_texture_back, getWindowWidth(), getWindowHeight(), normalization_transform);
    
    if(shadow_intensity)
        gfx::drawShadow(rect, shadow_intensity);
    
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
