#ifndef worldSelector_hpp
#define worldSelector_hpp

#include "graphics.hpp"
#include "menuBack.hpp"

struct WorldToSelect {
    std::string name;
    explicit WorldToSelect(std::string name) : name(std::move(name)) {}
    gfx::Button button, delete_button;
    void render(int position);
    int button_y{};
};

class WorldSelector : public gfx::Scene {
    gfx::Sprite title;
    gfx::Button back_button, new_button;
    std::vector<std::string> worlds_names;
    std::vector<WorldToSelect> worlds;
    int scroll_limit, position;
    BackgroundRect* menu_back;
    gfx::Rect top_rect, bottom_rect;
    float top_rect_visibility = 0;
    float bottom_rect_visibility = 0;
    
    void init() override;
    void onKeyDown(gfx::Key key) override;
    void render() override;
    void onMouseScroll(int distance) override;
    void refresh();
public:
    WorldSelector(BackgroundRect* menu_back) : menu_back(menu_back) {}
};

#endif
