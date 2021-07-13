//
//  graphics-internal.h
//  Terralistic
//
//  Created by Jakob Zorz on 09/03/2021.
//

#ifndef graphics_internal_hpp
#define graphics_internal_hpp



#ifdef _WIN32
#include <SDL.h>
#include <SDL_image.h>
#include <SDL_ttf.h>
#elif defined(__APPLE__)
#include <SDL2/SDL.h>
#include <SDL2_image/SDL_image.h>
#include <SDL2_ttf/SDL_ttf.h>
#else
#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include <SDL2/SDL_ttf.h>
#endif

#include "color/color.hpp"
#include "rect/rect.hpp"
#include "scene/scene.hpp"
#include "ui/ui.hpp"

namespace gfx {

inline SDL_Window* window = nullptr;
inline SDL_Renderer* renderer = nullptr;
inline TTF_Font *font = nullptr;

inline unsigned short mouse_x, mouse_y;
inline unsigned short window_height = 100, window_width = 100;
inline float frame_length;

}

#endif /* graphics_internal_hpp */
