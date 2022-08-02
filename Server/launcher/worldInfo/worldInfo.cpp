#include "worldInfo.hpp"

WorldInfo::WorldInfo(float x_, float y_, float w_, float h_){
    target_x = x_;
    target_y = y_;
    target_w = w_;
    target_h = h_;
    min_width = 300;
    min_height = 90;
    texture.createBlankImage(width, height);
}

void WorldInfo::init() {
    std::string data;
    data = server->world_path;
    size_t pos;
    while ((pos = data.find('/')) != std::string::npos)
        data.erase(0, pos + 1);
    name_text.loadFromText("Name: " + data);
    seed_text.loadFromText("Seed: " + std::to_string(server->seed));
    port_text.loadFromText("Port: " + std::to_string(server->getNetworking()->port));
    name_text.scale = 2;
    seed_text.scale = 2;
    port_text.scale = 2;
}



void WorldInfo::render() {
    if(width != texture.getTextureWidth() || height != texture.getTextureHeight())
        texture.createBlankImage(width, height);

    gfx::RectShape(2, 2, width - 4, height - 4).render(GREY);
    name_text.x = 10;
    name_text.y = 10;
    seed_text.x = 10;
    seed_text.y = (float)texture.getTextureHeight() - 10 - (float)seed_text.getHeight();
    port_text.x = (float)texture.getTextureWidth() / 2 - (float)port_text.getWidth() / 2;
    port_text.y = 10;

    texture.setRenderTarget();

    name_text.render();
    seed_text.render();
    port_text.render();

    gfx::resetRenderTarget();
}