#ifndef graphics_internal_hpp
#define graphics_internal_hpp

#include "graphics.hpp"

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

#include <SFML/Graphics.hpp>

namespace gfx {

inline SDL_Window* window = nullptr;
inline SDL_Renderer* renderer = nullptr;
inline TTF_Font *font = nullptr;

inline sf::RenderWindow *sfml_window = nullptr;
inline sf::Font sfml_font;

inline unsigned short mouse_x, mouse_y;
inline float frame_length;

}

#endif /* graphics_internal_hpp */
