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

int mouseOnMapX();
int mouseOnMapY();

int onMapX(unsigned int x);
int onMapY(unsigned int y);

void init();
void render();

inline int selectedBlockX, selectedBlockY;

void handleEvent(SDL_Event& event);

}

#endif /* blockSelector_hpp */
