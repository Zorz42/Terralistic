#include "worldInfo.hpp"

WorldInfo::WorldInfo(float x_, float y_, float w_, float h_){
    target_x = x_;
    target_y = y_;
    target_w = w_;
    target_h = h_;
    min_width = 100;
    min_height = 50;
    name_text.loadFromText("Name: ");
    name_text.x = 10;
    name_text.y = 10;
    name_text.scale = 2;
    seed_text.loadFromText("Seed: ");
    seed_text.orientation = gfx::BOTTOM_LEFT;
    seed_text.y = 100;
    seed_text.scale = 2;
    texture.createBlankImage(width, height);
}



void WorldInfo::render() {
    if(width != texture.getTextureWidth() || height != texture.getTextureHeight())
        texture.createBlankImage(width, height);
    texture.setRenderTarget();

    gfx::RectShape(0, 0, width, height).render({100, 100, 100, 255});
    name_text.render();
    seed_text.render();

    gfx::resetRenderTarget();
}