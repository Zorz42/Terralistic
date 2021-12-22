#include "graphics-internal.hpp"

void gfx::TextureAtlas::create(const std::vector<Texture*>& textures) {
    int width = 0, height = 0;
    for(int i = 0; i < textures.size(); i++) {
        height += textures[i]->getTextureHeight();
        width = std::max(width, textures[i]->getTextureWidth());
    }
    texture.createBlankImage(width, height);
    setRenderTarget(texture);
    rects.resize(textures.size());
    int y = 0;
    for(int i = 0; i < textures.size(); i++) {
        textures[i]->render(1, 0, y);
        rects[i] = RectShape(0, y, textures[i]->getTextureWidth(), textures[i]->getTextureHeight());
        y += textures[i]->getTextureHeight();
    }
    resetRenderTarget();
}

gfx::RectShape gfx::TextureAtlas::getRect(int id) {
    return rects[id];
}
