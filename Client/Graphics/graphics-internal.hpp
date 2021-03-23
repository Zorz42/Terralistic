//
//  graphics-internal.h
//  Terralistic
//
//  Created by Jakob Zorz on 09/03/2021.
//

#ifndef graphics_internal_hpp
#define graphics_internal_hpp

#include "graphics.hpp"

#include <SDL2/SDL.h>
#include <SDL2_image/SDL_image.h>
#include <SDL2_ttf/SDL_ttf.h>
#include <stack>

namespace gfx {

inline SDL_Window* window = nullptr;
inline SDL_Renderer* renderer = nullptr;
inline TTF_Font *font = nullptr;

inline std::stack<scene*> scene_stack;

inline unsigned short mouse_x, mouse_y;
inline unsigned short window_height = 100, window_width = 100;
inline float frame_length;

}

#endif /* graphics_internal_hpp */
