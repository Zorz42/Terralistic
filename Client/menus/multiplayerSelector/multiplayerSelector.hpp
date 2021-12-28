#pragma once
#include "menuBack.hpp"
#include "settings.hpp"

struct MenuServer {
    std::string ip;
    gfx::Button join_button, remove_button;
    gfx::Texture ip_texture, icon;
    void render(int position, int mouse_x, int mouse_y);
    int y;
};

class MultiplayerSelector : public gfx::Scene {
    void init() override;
    void onMouseScroll(int distance) override;
    bool onKeyDown(gfx::Key key) override;
    void render() override;
    void stop() override;
    
    gfx::Sprite title;
    gfx::Button back_button, new_button;
    gfx::Rect top_rect, bottom_rect;
    float top_rect_visibility = 0;
    
    std::vector<MenuServer*> servers;
    int scroll_limit = 0, position = 0;
    
    void addServer(const std::string& ip);

    BackgroundRect* menu_back;
    Settings* settings;
public:
    explicit MultiplayerSelector(BackgroundRect* menu_back, Settings* settings) : menu_back(menu_back), settings(settings) {}
};
