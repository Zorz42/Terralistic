//
//  blockSelector.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/07/2020.
//

#ifndef blockSelector_hpp
#define blockSelector_hpp

#include <SDL2/SDL.h>

namespace blockSelector {

void render();
bool collidingWithPlayer();

inline unsigned short selected_block_x, selected_block_y;

}

#endif /* blockSelector_hpp */
