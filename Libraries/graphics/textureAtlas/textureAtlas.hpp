#pragma once
#include <vector>
#include "texture.hpp"

namespace gfx {

class TextureAtlas {
    Texture texture;
    std::vector<RectShape> rects;
public:
    const Texture& getTexture() { return texture; }
    void create(const std::vector<Surface>& surfaces);
    RectShape getRect(int id);
};

};
