#pragma once
#include "color.hpp"
#include "rectShape.hpp"
#include "orientedObject.hpp"
#include "texture.hpp"

namespace gfx {

class Sprite : public _OrientedObject, public Texture {
    Color color{255, 255, 255};
    RectShape src_rect;
    float scale = 1;
public:
    Sprite();
    
    bool flipped = false;
    
    void setScale(float scale);
    float getScale() const;
    void setColor(Color color_);
    void render() const;
    void setSrcRect(RectShape src_rect);
    void createBlankImage(int width, int height);
    void loadFromSurface(const Surface& surface);
};

};
