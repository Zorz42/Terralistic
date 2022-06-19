#pragma once

#ifndef GRAPHICS_PUBLIC
#include "texture.hpp"

namespace gfx {

void initShadow();
void quitShadow();

inline Texture* shadow_texture = nullptr;

};

#endif
