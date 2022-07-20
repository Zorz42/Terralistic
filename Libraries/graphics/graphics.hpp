#pragma once

#define GRAPHICS_PUBLIC
#include "color.hpp"
#include "glfwAbstraction.hpp"
#include "transformation.hpp"
#include "surface.hpp"
#include "blur.hpp"
#include "rectShape.hpp"
#include "orientation.hpp"
#include "centeredObject.hpp"
#include "timer.hpp"
#include "font.hpp"
#include "texture.hpp"
#include "rectArray.hpp"
#include "rect.hpp"
#include "textureAtlas.hpp"
#include "sprite.hpp"
#include "button.hpp"
#include "textInput.hpp"
#include "scene.hpp"

namespace gfx {

void init(int window_width, int window_height, const std::string& window_title);
void quit();

};
