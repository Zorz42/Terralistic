#ifndef graphics_internal_hpp
#define graphics_internal_hpp

#include "graphics.hpp"

#include <SFML/Graphics.hpp>
#include <SFML/Window.hpp>

// higher number = faster, but worse quality
#define BLUR_QUALITY 2

namespace gfx {

inline sf::RenderWindow *window = nullptr;
inline sf::Font font;

inline unsigned short mouse_x, mouse_y;
inline float frame_length;

inline unsigned short font_size;
inline sf::RenderTexture *render_target = nullptr;

inline float global_scale = 1;

inline sf::Shader blur_shader;

void blurTexture(sf::RenderTexture& texture, float blur_intensity, int quality);
void blurRegion(sf::RenderTexture& target, RectShape region, float blur_intensity, int quality=BLUR_QUALITY);

}

#endif
