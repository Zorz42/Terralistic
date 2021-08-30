#include "graphics-internal.hpp"


gfx::Color::Color(unsigned char r, unsigned char g, unsigned char b, unsigned char a) : r(r), g(g), b(b), a(a) {}

gfx::RectShape::RectShape(short x, short y, unsigned short w, unsigned short h) : x(x), y(y), w(w), h(h) {}

void gfx::RectShape::render(Color c, bool fill) const {
    sf::RectangleShape rect(sf::Vector2f(w, h));
    rect.setPosition(x, y);
    if (fill) {
        rect.setOutlineColor(sf::Color::Transparent);
        rect.setFillColor((const sf::Color)c);
        render_target->draw(rect);
    } else {
        rect.setFillColor(sf::Color::Transparent);
        rect.setOutlineColor((const sf::Color)c);
        rect.setOutlineThickness(1);
        render_target->draw(rect);
    }
}


gfx::RectShape gfx::_CenteredObject::getTranslatedRect() const {
    return {getTranslatedX(), getTranslatedY(), getWidth(), getHeight()};
}  
short gfx::_CenteredObject::getTranslatedX(unsigned short width) const {
    if(!width)
        width = getWidth();
    return (orientation % 3 == 1 ? (getWindowWidth() >> 1) - (width >> 1) : (orientation % 3 == 2 ? getWindowWidth() - width : 0)) + x;
}

short gfx::_CenteredObject::getTranslatedY(unsigned short height) const {
    if(!height)
        height = getHeight();
    return (orientation / 3 == 1 ? (getWindowHeight() >> 1) - (height >> 1) : (orientation / 3 == 2 ? getWindowHeight() - height : 0)) + y;
}

gfx::_CenteredObject::_CenteredObject(short x, short y, ObjectType orientation) : orientation(orientation), x(x), y(y) {}

gfx::Rect::Rect(short x, short y, unsigned short w, unsigned short h, Color c, ObjectType orientation) : _CenteredObject(0, 0, orientation), c(c) {
    setX(x);
    setY(y);
    setWidth(w);
    setHeight(h);
}

int approach(int object, int target, int smooth_factor) {
    if(std::abs(target - object) < smooth_factor)
        return target;
    return object + (target - object) / smooth_factor;
}

void gfx::Rect::render(bool fill) {
    if(first_time) {
        first_time = false;
        x = target_x;
        y = target_y;
        width = target_width;
        height = target_height;
    }
    
    bool changed = false;
    
    short prev_x = x;
    x = approach(x, target_x, smooth_factor);
    if(prev_x != x)
        changed = true;
    
    short prev_y = y;
    y = approach(y, target_y, smooth_factor);
    if(prev_y != y)
        changed = true;
    
    unsigned short prev_width = width;
    width = approach(width, target_width, smooth_factor);
    if(prev_width != width)
        changed = true;
    
    unsigned short prev_height = height;
    height = approach(height, target_height, smooth_factor);
    if(prev_height != height)
        changed = true;
    
    RectShape rect = getTranslatedRect();

    if(blur_intensity) {
        if(blur_intensity && !blur_texture) {
            blur_texture = new sf::RenderTexture;
            updateBlurTextureSize();
        }
        
        blur_texture->clear({0, 0, 0});
        sf::Sprite back_sprite;
        back_sprite.setTexture(render_target->getTexture());
        back_sprite.setTextureRect({getTranslatedX() - x + target_x, getTranslatedY() - y + target_y, target_width, target_height});
        blur_texture->draw(back_sprite);
        blur_texture->display();
        
        blurTexture(*blur_texture, blur_intensity);
        blur_texture->display();
        
        sf::Sprite sprite;
        sprite.setTexture(blur_texture->getTexture());
        sprite.setPosition(getTranslatedX(), getTranslatedY());
        sprite.setTextureRect({0, 0, width, height});
        render_target->draw(sprite);
    }
    
    if(shadow_intensity) {
        if(!shadow_texture) {
            shadow_texture = new sf::RenderTexture;
            updateShadowTexture();
        }
        
        if(getWindowWidth() != shadow_texture->getSize().x || getWindowHeight() != shadow_texture->getSize().y)
            updateShadowTexture();
        
        if(changed)
            updateShadowTexture();
        
        //render_target->draw(sf::Sprite(shadow_texture->getTexture()));
        
        sf::Sprite shadow_sprite(gfx::shadow_texture.getTexture());
        unsigned short temp_width, temp_height;
        
        temp_width = std::min(200 + width / 2, 350);
        temp_height = std::min(200 + height / 2, 350);
        
        shadow_sprite.setTextureRect({0, 0, temp_width, temp_height});
        shadow_sprite.setPosition(getTranslatedX() - 200, getTranslatedY() - 200);
        render_target->draw(shadow_sprite);
        
        shadow_sprite.setTextureRect({700 - temp_width, 0, temp_width, temp_height});
        shadow_sprite.setPosition(getTranslatedX() + width - temp_width + 200, getTranslatedY() - 200);
        render_target->draw(shadow_sprite);
        
        shadow_sprite.setTextureRect({0, 700 - temp_height, temp_width, temp_height});
        shadow_sprite.setPosition(getTranslatedX() - 200, getTranslatedY() + height - temp_height + 200);
        render_target->draw(shadow_sprite);
        
        shadow_sprite.setTextureRect({700 - temp_width, 700 - temp_height, temp_width, temp_height});
        shadow_sprite.setPosition(getTranslatedX() + width - temp_width + 200, getTranslatedY() + height - temp_height + 200);
        render_target->draw(shadow_sprite);
        
        if(temp_height == 350) {
            sf::Sprite shadow_part_sprite_left(shadow_part_left.getTexture());
            shadow_part_sprite_left.setTextureRect({0, 0, 200, height - 300});
            shadow_part_sprite_left.setPosition(getTranslatedX() - 200, getTranslatedY() + 150);
            render_target->draw(shadow_part_sprite_left);
            
            sf::Sprite shadow_part_sprite_right(shadow_part_right.getTexture());
            shadow_part_sprite_right.setTextureRect({0, 0, 200, height - 300});
            shadow_part_sprite_right.setPosition(getTranslatedX() + width, getTranslatedY() + 150);
            render_target->draw(shadow_part_sprite_right);
        }
        
        if(temp_width == 350) {
            sf::Sprite shadow_part_sprite_up(shadow_part_up.getTexture());
            shadow_part_sprite_up.setTextureRect({0, 0, width - 300, 200});
            shadow_part_sprite_up.setPosition(getTranslatedX() + 150, getTranslatedY() - 200);
            render_target->draw(shadow_part_sprite_up);
            
            sf::Sprite shadow_part_sprite_down(shadow_part_down.getTexture());
            shadow_part_sprite_down.setTextureRect({0, 0, width - 300, 200});
            shadow_part_sprite_down.setPosition(getTranslatedX() + 150, getTranslatedY() + height);
            render_target->draw(shadow_part_sprite_down);
        }
        
        
        //shadow_sprite.setPosition(getTranslatedX() - 200, getTranslatedY() + 200);
        //render_target->draw(shadow_sprite);
    }
    rect.render(c, fill);
    rect.render(border_color, false);
}

void gfx::Rect::updateShadowTexture() {
    if(!shadow_texture)
        return;
    if(getWindowWidth() != shadow_texture->getSize().x || getWindowHeight() != shadow_texture->getSize().y)
        shadow_texture->create(getWindowWidth(), getWindowHeight());
    
    shadow_texture->clear({0, 0, 0, 0});
    
    sf::RectangleShape rect(sf::Vector2f(width, height));
    rect.setPosition(getTranslatedX(), getTranslatedY());
    rect.setFillColor({0, 0, 0, shadow_intensity});
    shadow_texture->draw(rect);
    
    shadow_texture->display();
    
    blurTexture(*shadow_texture, shadow_blur, 2);
    
    rect.setFillColor({0, 0, 0, 0});
    shadow_texture->draw(rect, sf::BlendNone);
    
    shadow_texture->display();
}

void gfx::Rect::updateBlurTextureSize() {
    if(blur_texture)
        blur_texture->create(target_width, target_height);
}

unsigned short gfx::Rect::getWidth() const {
    return width;
}

void gfx::Rect::setWidth(unsigned short width_) {
    if(target_width != width_) {
        target_width = width_;
        updateBlurTextureSize();
        updateShadowTexture();
    }
}

unsigned short gfx::Rect::getHeight() const {
    return height;
}

void gfx::Rect::setHeight(unsigned short height_) {
    if(target_height != height_) {
        target_height = height_;
        updateBlurTextureSize();
        updateShadowTexture();
    }
}

short gfx::Rect::getX() const {
    return x;
}

void gfx::Rect::setX(short x_) {
    if(target_x != x_) {
        target_x = x_;
        updateShadowTexture();
    }
}

short gfx::Rect::getY() const {
    return y;
}

void gfx::Rect::setY(short y_) {
    if(target_y != y_) {
        target_y = y_;
        updateShadowTexture();
    }
}

gfx::Rect::~Rect() {
    delete shadow_texture;
    delete blur_texture;
}
