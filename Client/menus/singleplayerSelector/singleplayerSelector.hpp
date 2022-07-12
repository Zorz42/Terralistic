#pragma once
#include "menuBack.hpp"
#include "settings.hpp"

struct World {
    std::string name;
    gfx::Button play_button, delete_button;
    gfx::Texture title, icon, last_played;
    void render(int position, int mouse_x, int mouse_y);
    int y;
};

class SingleplayerSelector : public gfx::Scene {
    gfx::Sprite title;
    gfx::Button back_button, new_button;
    gfx::Rect top_rect, bottom_rect;
    float top_rect_visibility = 0;
    
    std::vector<World*> worlds;
    int scroll_limit = 0, position = 0;
    
    void init() override;
    bool onKeyUp(gfx::Key key) override;
    void render() override;
    void onMouseScroll(int distance) override;
    void refresh();
    
    BackgroundRect* menu_back;
    Settings* settings;
public:
    explicit SingleplayerSelector(BackgroundRect* menu_back, Settings* settings) : gfx::Scene("SinglelpayerSelector"), menu_back(menu_back), settings(settings) {}
};
