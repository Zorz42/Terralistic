#include "worldInfo.hpp"
#include <chrono>
#include <ctime>
#include <iomanip>

WorldInfo::WorldInfo(std::string resource_path): LauncherModule("world_info", std::move(resource_path)){
    min_width = 620;
    min_height = 90;
}

void WorldInfo::init() {
    std::string data;
    data = server->world_path;
    size_t pos;
    while ((pos = data.find('/')) != std::string::npos)
        data.erase(0, pos + 1);
    name_text.loadFromSurface(gfx::textToSurface("Name: " + data));
    seed_text.loadFromSurface(gfx::textToSurface("Seed: " + std::to_string(server->seed)));
    port_text.loadFromSurface(gfx::textToSurface("Port: " + std::to_string(server->getNetworking()->port)));
    name_text.setScale(2);
    seed_text.setScale(2);
    port_text.setScale(2);
    state_text.setScale(2);
    clock_text.setScale(2);

    name_text.parent_containter = &base_container;
    seed_text.parent_containter = &base_container;
    port_text.parent_containter = &base_container;
    state_text.parent_containter = &base_container;
    clock_text.parent_containter = &base_container;
    port_text.orientation = gfx::TOP;
    clock_text.orientation = gfx::TOP_RIGHT;
    seed_text.orientation = gfx::BOTTOM_LEFT;
    state_text.orientation = gfx::BOTTOM;
    name_text.x = 10;
    name_text.y = 10;
    seed_text.x = 10;
    seed_text.y = -10;
    port_text.x = 0;
    port_text.y = 10;
    state_text.x = 0;
    state_text.y = -10;
    clock_text.x = -10;
    clock_text.y = 10;
}



void WorldInfo::render() {
    if(lastState != server->state) {
        switch((int) server->state) {
            case 0:
                state_text.loadFromSurface(gfx::textToSurface("Server state: neutral"));
                break;
            case 1:
                state_text.loadFromSurface(gfx::textToSurface("Server state: loading world"));
                break;
            case 2:
                state_text.loadFromSurface(gfx::textToSurface("Server state: generating world"));
                break;
            case 3:
                state_text.loadFromSurface(gfx::textToSurface("Server state: running"));
                break;
            case 4:
                state_text.loadFromSurface(gfx::textToSurface("Server state: stopping"));
                break;
            case 5:
                state_text.loadFromSurface(gfx::textToSurface("Server state: stopped"));
                break;
            case 6:
                state_text.loadFromSurface(gfx::textToSurface("Server state: crashed"));
                break;
            default:
                state_text.loadFromSurface(gfx::textToSurface("This text did not load properly"));
                break;
        }
        lastState = server->state;
    }

    auto t = std::time(nullptr);
    auto tm = *localtime(&t);
    std::stringstream time1;
    time1 << std::put_time(&tm, "%d.%m.%Y %H:%M:%S");
    clock_text.loadFromSurface(gfx::textToSurface(time1.str()));

    gfx::RectShape(base_container.x + 2, base_container.y + 2, base_container.w - 4, base_container.h - 4).render(GREY);
    name_text.render();
    seed_text.render();
    port_text.render();
    state_text.render();
    clock_text.render();
}