#include "graphics-internal.hpp"


gfx::Color::Color(unsigned char r, unsigned char g, unsigned char b, unsigned char a) : r(r), g(g), b(b), a(a) {}


gfx::RectShape::RectShape(short x, short y, unsigned short w, unsigned short h) : x(x), y(y), w(w), h(h) {}

void gfx::RectShape::render(Color c, bool fill) {
    sf::RectangleShape rec(sf::Vector2f(w, h));
    rec.setPosition(x, y);
    if (fill) {
        rec.setOutlineColor(sf::Color::Transparent);
        rec.setFillColor((const sf::Color)c);
        gfx::sfml_window->draw(rec);
    }
    else {
        rec.setFillColor(sf::Color::Transparent);
        rec.setOutlineColor((const sf::Color)c);
        rec.setOutlineThickness(5.0f);
        gfx::sfml_window->draw(rec);
    }
        
    


    SDL_SetRenderDrawColor(gfx::renderer, c.r, c.g, c.b, c.a);
    SDL_Rect sdl_rect = { this->x, this->y, this->w, this->h };
    if (fill)
        SDL_RenderFillRect(gfx::renderer, &sdl_rect);
    else
        SDL_RenderDrawRect(gfx::renderer, &sdl_rect); 
}


gfx::RectShape gfx::_CenteredObject::getTranslatedRect() const {
    return RectShape(getTranslatedX(), getTranslatedY(), getWidth(), getHeight());
}  
short gfx::_CenteredObject::getTranslatedX() const {
    return orientation % 3 == 1 ? (window_width >> 1) - (getWidth() >> 1) + x : (orientation % 3 == 2 ? window_width - getWidth() + x : x);
}

short gfx::_CenteredObject::getTranslatedY() const {
    return orientation / 3 == 1 ? (window_height >> 1) - (getHeight() >> 1) + y : (orientation / 3 == 2 ? window_height - getHeight() + y : y);
}

gfx::_CenteredObject::_CenteredObject(short x, short y, ObjectType orientation) : orientation(orientation), x(x), y(y) {}

gfx::Rect::Rect(short x, short y, unsigned short w, unsigned short h, Color c, ObjectType orientation) : _CenteredObject(x, y, orientation), w(w), h(h), c(c) {}

void gfx::Rect::render(bool fill) const {
    //short x, y, w, h;
    RectShape gfx_rect = getTranslatedRect();
    sf::RectangleShape rec(sf::Vector2f(gfx_rect.w, gfx_rect.h));
    rec.setPosition(gfx_rect.x, gfx_rect.y);
    if(fill) {
        rec.setOutlineColor(sf::Color::Transparent);
        rec.setFillColor((const sf::Color)c);
        gfx::sfml_window->draw(rec);
    } else {
        rec.setFillColor(sf::Color::Transparent);
        rec.setOutlineColor((const sf::Color)c);
        rec.setOutlineThickness(5.0f);
        gfx::sfml_window->draw(rec);
    }



    SDL_SetRenderDrawColor(gfx::renderer, this->c.r, this->c.g, this->c.b, this->c.a);
    SDL_Rect sdl_rect = { gfx_rect.x, gfx_rect.y, gfx_rect.w, gfx_rect.h };
    if (fill)
        SDL_RenderFillRect(gfx::renderer, &sdl_rect);
    else
        SDL_RenderDrawRect(gfx::renderer, &sdl_rect);
}
