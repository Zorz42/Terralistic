#include <cmath>
#include "graphics-internal.hpp"

int approach(int object, int target, int smooth_factor) {
    if(std::abs(target - object) < smooth_factor)
        return target;
    return object + (target - object) / smooth_factor;
}

void gfx::Rect::render() {
    first_time = false;
    
    if(approach_timer.getTimeElapsed() > 14) {
        x = approach(x, target_x, smooth_factor);
        y = approach(y, target_y, smooth_factor);
        width = approach(width, target_width, smooth_factor);
        height = approach(height, target_height, smooth_factor);
        approach_timer.reset();
    }
    
    RectShape rect = getTranslatedRect();

    if(blur_radius && blur_enabled)
        gfx::blurRectangle(rect, blur_radius, window_texture, window_texture_back, getWindowWidth(), getWindowHeight(), normalization_transform);

    rect.render(fill_color);
    rect.renderOutline(border_color);
    
    /*if(shadow_intensity) {
        sf::Sprite shadow_sprite(gfx::shadow_texture.getTexture());
        shadow_sprite.setColor({255, 255, 255, shadow_intensity});
        
        float temp_width = std::min(200.f + (float)width / 2, 350.f), temp_height = std::min(200.f + (float)height / 2, 350.f);
        
        shadow_sprite.setTextureRect({0, 0, (int)std::floor(temp_width), (int)std::ceil(temp_height)});
        shadow_sprite.setPosition(getTranslatedX() - 200, getTranslatedY() - 200);
        render_target->draw(shadow_sprite);
        
        shadow_sprite.setTextureRect({700 - (int)std::ceil(temp_width), 0, (int)std::ceil(temp_width), (int)std::ceil(temp_height)});
        shadow_sprite.setPosition(getTranslatedX() + width - std::ceil(temp_width) + 200, getTranslatedY() - 200);
        render_target->draw(shadow_sprite);
        
        shadow_sprite.setTextureRect({0, 700 - (int)std::floor(temp_height), (int)std::floor(temp_width), (int)std::floor(temp_height)});
        shadow_sprite.setPosition(getTranslatedX() - 200, getTranslatedY() + height - std::floor(temp_height) + 200);
        render_target->draw(shadow_sprite);
        
        shadow_sprite.setTextureRect({700 - (int)std::ceil(temp_width), 700 - (int)std::floor(temp_height), (int)std::ceil(temp_width), (int)std::floor(temp_height)});
        shadow_sprite.setPosition(getTranslatedX() + width - std::ceil(temp_width) + 200, getTranslatedY() + height - std::floor(temp_height) + 200);
        render_target->draw(shadow_sprite);
        
        if(temp_height == 350) {
            sf::Sprite shadow_part_sprite_left(shadow_part_left.getTexture());
            shadow_part_sprite_left.setTextureRect({0, 0, 200, height - 300});
            shadow_part_sprite_left.setPosition(getTranslatedX() - 200, getTranslatedY() + 150);
            shadow_part_sprite_left.setColor({255, 255, 255, shadow_intensity});
            render_target->draw(shadow_part_sprite_left);
            
            sf::Sprite shadow_part_sprite_right(shadow_part_right.getTexture());
            shadow_part_sprite_right.setTextureRect({0, 0, 200, height - 300});
            shadow_part_sprite_right.setPosition(getTranslatedX() + width, getTranslatedY() + 150);
            shadow_part_sprite_right.setColor({255, 255, 255, shadow_intensity});
            render_target->draw(shadow_part_sprite_right);
        }
        
        if(temp_width == 350) {
            sf::Sprite shadow_part_sprite_up(shadow_part_up.getTexture());
            shadow_part_sprite_up.setTextureRect({0, 0, width - 300, 200});
            shadow_part_sprite_up.setPosition(getTranslatedX() + 150, getTranslatedY() - 200);
            shadow_part_sprite_up.setColor({255, 255, 255, shadow_intensity});
            render_target->draw(shadow_part_sprite_up);
            
            sf::Sprite shadow_part_sprite_down(shadow_part_down.getTexture());
            shadow_part_sprite_down.setTextureRect({0, 0, width - 300, 200});
            shadow_part_sprite_down.setPosition(getTranslatedX() + 150, getTranslatedY() + height);
            shadow_part_sprite_down.setColor({255, 255, 255, shadow_intensity});
            render_target->draw(shadow_part_sprite_down);
        }

    }*/
    //rect.render(fill_color);
    //rect.renderOutline(border_color);
}

int gfx::Rect::getWidth() const {
    return width;
}

void gfx::Rect::setWidth(int width_) {
    if(width_ < 0)
        throw std::runtime_error("Width must be positive.");
    target_width = width_;
    if(first_time)
        width = width_;
}

int gfx::Rect::getHeight() const {
    return height;
}

void gfx::Rect::setHeight(int height_) {
    if(height_ < 0)
        throw std::runtime_error("Height must be positive.");
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
