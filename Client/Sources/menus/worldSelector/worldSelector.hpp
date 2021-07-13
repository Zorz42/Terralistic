//
//  worldSelector.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#ifndef worldSelector_hpp
#define worldSelector_hpp

#include <iostream>
#include "graphics.hpp"

class worldSelector : public gfx::Scene {
    struct world_to_select {
        std::string name;
        explicit world_to_select(std::string name) : name(std::move(name)) {}
        gfx::Button button, delete_button;
        void render(int position);
        int button_y{};
    };

    gfx::Sprite title;
    gfx::Image x_image;
    gfx::Button back_button, new_button;
    std::vector<std::string> worlds_names;
    std::vector<world_to_select> worlds;
    int scroll_limit, position;
    bool shift_pressed = false;
public:
    void init() override;
    void refresh();
    void onKeyDown(gfx::key key) override;
    void onKeyUp(gfx::key key) override;
    void render() override;
    void onMouseScroll(int distance) override;
};

#endif /* worldSelector_hpp */
