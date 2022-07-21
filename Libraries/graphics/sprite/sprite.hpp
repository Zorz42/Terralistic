#pragma once
#include "color.hpp"
#include "rectShape.hpp"
#include "centeredObject.hpp"
#include "texture.hpp"

namespace gfx {

class Sprite : public _CenteredObject, public Texture {
    Color color{255, 255, 255};
    RectShape src_rect;
public:
    Sprite();
    
    bool flipped = false;
    float scale = 1;
    int getWidth() const override;
    int getHeight() const override;
    void setColor(Color color_);
    void render() const;
    void setSrcRect(RectShape src_rect);
    void createBlankImage(int width, int height);
    void loadFromSurface(const Surface& surface);
    void loadFromText(const std::string &text, Color color={255, 255, 255});
};

};
