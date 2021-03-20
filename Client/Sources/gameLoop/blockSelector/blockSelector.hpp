//
//  blockSelector.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/07/2020.
//

#ifndef blockSelector_hpp
#define blockSelector_hpp

#include <Graphics/graphics.hpp>

namespace blockSelector {

void render();
bool collidingWithPlayer();
void onKeyDown(gfx::key key);
void onKeyUp(gfx::key key);

inline unsigned short selected_block_x, selected_block_y;

struct clickEvents {
    void (*rightClickEvent)(blockEngine::block*) = nullptr;
    void (*leftClickEvent)(blockEngine::block*) = nullptr;
};

inline std::vector<clickEvents> click_events;

}

#endif /* blockSelector_hpp */
