#pragma once
#include "color.hpp"
#include "rectShape.hpp"
#include "orientedObject.hpp"
#include "texture.hpp"

namespace gfx {

class Sprite : public _OrientedObject, public Texture {
    using gfx::_OrientedObject::w;
    using gfx::_OrientedObject::h;
    
    Color color{255, 255, 255};
    RectShape src_rect;
    float scale = 1;
    
    void updateSize();
public:
    Sprite();
    
    bool flipped = false;
    
    void setScale(float scale);
    float getScale() const;
    void setColor(Color color_);
    
    void render() const;
    
    void loadFromSurface(const Surface& surface);
    
    int getWidth() const;
    int getHeight() const;
};

};
