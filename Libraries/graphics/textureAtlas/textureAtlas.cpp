#include "textureAtlas.hpp"

void gfx::TextureAtlas::create(const std::vector<Surface>& surfaces) {
    int width = 0, height = 0;
    for(auto& surface : surfaces) {
        height += surface.getHeight();
        width = std::max(width, surface.getWidth());
    }
    Surface atlas_surface;
    atlas_surface.createEmpty(width, height);
    rects.resize(surfaces.size());
    int y = 0;
    for(int i = 0; i < surfaces.size(); i++) {
        atlas_surface.draw(0, y, surfaces[i]);
        rects[i] = RectShape(0, y, surfaces[i].getWidth(), surfaces[i].getHeight());
        y += surfaces[i].getHeight();
    }
    texture.loadFromSurface(atlas_surface);
}

gfx::RectShape gfx::TextureAtlas::getRect(int id) {
    return rects[id];
}
