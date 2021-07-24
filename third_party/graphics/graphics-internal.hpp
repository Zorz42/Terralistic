#ifndef graphics_internal_hpp
#define graphics_internal_hpp

#include "graphics.hpp"

#include <SFML/Graphics.hpp>

namespace gfx {

inline sf::RenderWindow *sfml_window = nullptr;
inline sf::Font sfml_font;

inline unsigned short mouse_x, mouse_y;
inline float frame_length;

inline unsigned short font_size;
inline sf::RenderTarget *render_target = nullptr;
inline sf::Clock clock;

inline unsigned short min_window_width, min_window_height;

inline float global_scale = 1;

}

#endif /* graphics_internal_hpp */
