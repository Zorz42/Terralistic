#include "worldInfo.hpp"
#include <chrono>
#include <ctime>

WorldInfo::WorldInfo(float x_, float y_, float w_, float h_): LauncherModule("world_info"){
    target_x = x_;
    target_y = y_;
    target_w = w_;
    target_h = h_;
    min_width = 620;
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
    state_text.scale = 2;
    clock_text.scale = 2;
}



void WorldInfo::update(float frame_length) {
    if(width != texture.getTextureWidth() || height != texture.getTextureHeight())
        texture.createBlankImage(width, height);

    if(lastState != server->state) {
        switch((int) server->state) {
            case 0:
                state_text.loadFromText("Server state: neutral");
                break;
            case 1:
                state_text.loadFromText("Server state: loading world");
                break;
            case 2:
                state_text.loadFromText("Server state: generating world");
                break;
            case 3:
                state_text.loadFromText("Server state: running");
                break;
            case 4:
                state_text.loadFromText("Server state: stopping");
                break;
            case 5:
                state_text.loadFromText("Server state: stopped");
                break;
            case 6:
                state_text.loadFromText("Server state: crashed");
                break;
            default:
                state_text.loadFromText("This text did not load properly");
                break;
        }
        lastState = server->state;
    }

    auto time = std::chrono::system_clock::now();
    std::time_t time_ = std::chrono::system_clock::to_time_t(time);
    std::string time_string = std::ctime(&time_);
    clock_text.loadFromText(time_string.substr(0, time_string.size() - 6));

    name_text.x = 10;
    name_text.y = 10;
    seed_text.x = 10;
    seed_text.y = (float)texture.getTextureHeight() - 10 - (float)seed_text.getHeight();
    port_text.x = (float)texture.getTextureWidth() / 2 - (float)port_text.getWidth() / 2;
    port_text.y = 10;
    state_text.x = (float)texture.getTextureWidth() / 2 - (float)state_text.getWidth() / 2;
    state_text.y = (float)texture.getTextureHeight() - 10 - (float)state_text.getHeight();
    clock_text.x = (float)texture.getTextureWidth() - 10 - (float)clock_text.getWidth();
    clock_text.y = 10;

    texture.setRenderTarget();

    gfx::RectShape(2, 2, width - 4, height - 4).render(GREY);
    name_text.render();
    seed_text.render();
    port_text.render();
    state_text.render();
    clock_text.render();

    gfx::resetRenderTarget();
}