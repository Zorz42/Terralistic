#pragma once

namespace gfx {

class TextureAtlas {
    Texture texture;
    std::vector<RectShape> rects;
public:
    const Texture& getTexture() { return texture; }
    void create(const std::vector<Texture*>& textures);
    RectShape getRect(int id);
};

};
