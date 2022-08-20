#pragma once
#include <utility>

#include "menuBack.hpp"
#include "settings.hpp"

struct ServerData{
    std::string name;
    std::string ip;
    ServerData(std::string name, std::string ip) : name(std::move(name)), ip(std::move(ip)) {}
};

struct MenuServer {
    ServerData data;
    gfx::Button join_button, remove_button;
    gfx::Texture name_texture, icon;
    void render(int position, int mouse_x, int mouse_y, int mouse_vel, bool is_mouse_pressed);
    int y = 0;
    explicit MenuServer(ServerData data) : data(std::move(data)) {}
};


class MultiplayerSelector : public gfx::Scene {
    void init() override;
    void onMouseScroll(int distance) override;
    bool onKeyUp(gfx::Key key) override;
    void render() override;
    void stop() override;
    
    gfx::Sprite title;
    gfx::Button back_button, new_button;
    gfx::Rect top_rect, bottom_rect;
    float top_rect_visibility = 0;

    std::vector<ServerData> server_data;
    std::vector<MenuServer*> servers;
    int scroll_limit = 0, position = 0;
    
    void refresh();

    BackgroundRect* menu_back;
    Settings* settings;
public:
    MultiplayerSelector(BackgroundRect* menu_back, Settings* settings) : gfx::Scene("MultiplayerSelector"), menu_back(menu_back), settings(settings) {}
};
