#pragma once
#include "surface.hpp"
#include "theme.hpp"

namespace gfx {

void loadFont(const Surface& surface);
gfx::Surface textToSurface(const std::string& text, Color color=WHITE);
int getCharWidth(char c);

};
