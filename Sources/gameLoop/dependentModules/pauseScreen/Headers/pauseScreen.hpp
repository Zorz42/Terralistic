//
//  pauseScreen.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#ifndef pauseScreen_hpp
#define pauseScreen_hpp

#include <SDL2/SDL.h>

namespace pauseScreen {

void init();
void render();
void handleEvents(SDL_Event& event);
inline bool paused = false;

}

#endif /* pauseScreen_hpp */
