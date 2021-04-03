//
//  worldSelector.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#ifndef worldSelector_hpp
#define worldSelector_hpp

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include <iostream>

struct worldSelector : public gfx::scene {
    void init() override;
    void refresh() override;
    void onKeyDown(gfx::key key) override;
    void render() override;
    void onMouseScroll(int distance) override;
    
private:
    struct world_to_select {
        std::string name;
        explicit world_to_select(std::string name) : name(std::move(name)) {}
        gfx::button button, delete_button;
        void render(int position);
        int button_y{};
    };
    
    gfx::sprite title;
    gfx::image x_image;
    gfx::button back_button, new_button;
    std::vector<std::string> worlds_names;
    std::vector<world_to_select> worlds;
    int scroll_limit, position;
};

#endif /* worldSelector_hpp */
