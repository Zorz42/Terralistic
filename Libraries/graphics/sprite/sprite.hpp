#pragma once
#include "color.hpp"
#include "rectShape.hpp"
#include "container.hpp"
#include "texture.hpp"

namespace gfx {

class Sprite : public Container, public Texture {
    using gfx::Container::w;
    using gfx::Container::h;
    
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
