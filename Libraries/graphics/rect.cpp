#include <cmath>
#include "graphics-internal.hpp"
#include "exception.hpp"

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

    if(blur_intensity && blur_enabled) {
        if(!blur_texture) {
            blur_texture = new sf::RenderTexture;
            updateBlurTextureSize();
        }
        
        if(blur_timer.getTimeElapsed() > 33) {
            updateBlurTexture();
            blur_timer.reset();
        }
        
        sf::Sprite sprite;
        sprite.setTexture(blur_texture->getTexture());
        sprite.setPosition(getTranslatedX(), getTranslatedY());
        sprite.setTextureRect({0, 0, width, height});
        render_target->draw(sprite);
    }
    
    if(shadow_intensity) {
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

    }
    rect.render(fill_color);
    rect.renderOutline(border_color);
}

void gfx::Rect::updateBlurTextureSize() {
    if(blur_texture && target_width && target_height && (blur_texture->getSize().x != target_width || blur_texture->getSize().y != target_height)) {
        if(!blur_texture->create(target_width, target_height))
            throw Exception("Could not create texture.");
        updateBlurTexture();
    }
}

void gfx::Rect::updateBlurTexture() {
    blur_texture->clear({0, 0, 0});
    sf::Sprite back_sprite;
    back_sprite.setTexture(render_target->getTexture());
    back_sprite.setTextureRect({getTranslatedX() - x + target_x, getTranslatedY() - y + target_y, target_width, target_height});
    blur_texture->draw(back_sprite);
    blur_texture->display();
    
    blurTexture(*blur_texture, blur_intensity);
    blur_texture->display();
}

int gfx::Rect::getWidth() const {
    return width;
}

void gfx::Rect::setWidth(int width_) {
    if(width_ < 0)
        throw Exception("Width must be positive.");
    
    if(target_width != width_) {
        target_width = width_;
        updateBlurTextureSize();
    }
    if(first_time)
        width = width_;
}

int gfx::Rect::getHeight() const {
    return height;
}

void gfx::Rect::setHeight(int height_) {
    if(height_ < 0)
        throw Exception("Height must be positive.");
    
    if(target_height != height_) {
        target_height = height_;
        updateBlurTextureSize();
    }
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

gfx::Rect::~Rect() {
    delete blur_texture;
}
