#pragma once

namespace gfx {

void loadFont(const unsigned char* data);


};

#ifndef GRAPHICS_PUBLIC
#include "texture.hpp"

namespace gfx {

void quitFont();

inline Texture* font_texture = nullptr;
inline RectShape font_rects[256];

};

#endif
