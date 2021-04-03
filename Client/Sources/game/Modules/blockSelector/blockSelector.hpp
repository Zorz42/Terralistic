//
//  blockSelector.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/07/2020.
//

#ifndef blockSelector_hpp
#define blockSelector_hpp

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include "game.hpp"

struct blockSelector : public gfx::sceneModule<game> {
    using gfx::sceneModule<game>::sceneModule;
    
    void render() override;
    void onKeyDown(gfx::key key) override;
    void onKeyUp(gfx::key key) override;
private:
    gfx::rect select_rect{0, 0, BLOCK_WIDTH, BLOCK_WIDTH, {255, 0, 0}};
    bool is_left_button_pressed = false;
    unsigned short prev_selected_x, prev_selected_y, selected_block_x, selected_block_y;
};

#endif /* blockSelector_hpp */
