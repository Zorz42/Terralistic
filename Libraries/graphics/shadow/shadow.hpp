#pragma once

#ifndef GRAPHICS_PUBLIC
#include "texture.hpp"

namespace gfx {

void initShadow();
void quitShadow();

void drawShadow(RectShape rect, unsigned char shadow_intensity);

inline Texture* shadow_texture = nullptr;

};

#endif
