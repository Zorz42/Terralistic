//
//  playerRenderer.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/04/2021.
//

#ifndef playerRenderer_hpp
#define playerRenderer_hpp

#ifdef _WIN32
#include "graphics.hpp"
#else

#ifdef DEVELOPER_MODE
#include <Graphics_Debug/graphics.hpp>
#else
#include <Graphics/graphics.hpp>
#endif

#endif

namespace playerRenderer {

void init();
void render(int x, int y, int view_x, int view_y, bool flipped);
void render(int x, int y, int view_x, int view_y, bool flipped, gfx::image& header);
unsigned short getPlayerWidth();
unsigned short getPlayerHeight();

}
#endif /* playerRenderer_hpp */
