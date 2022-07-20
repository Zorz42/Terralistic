#pragma once
#include "surface.hpp"

namespace gfx {

void loadFont(const Surface& surface);


};

#ifndef GRAPHICS_PUBLIC
#include "texture.hpp"

namespace gfx {

void quitFont();

inline Texture* font_texture = nullptr;
inline RectShape font_rects[256];

};

#endif
