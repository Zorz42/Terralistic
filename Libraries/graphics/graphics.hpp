#pragma once

#include <utility>
#include <vector>
#include <string>
#include <stdexcept>
#include <chrono>

#include "theme.hpp"

#define GRAPHICS_PUBLIC
#include "color.hpp"
#include "glfwAbstraction.hpp"
#include "rectShape.hpp"
#include "orientation.hpp"
#include "centeredObject.hpp"
#include "transformation.hpp"
#include "timer.hpp"
#include "texture.hpp"
#include "rectArray.hpp"
#include "rect.hpp"
#include "textureAtlas.hpp"
#include "sprite.hpp"
#include "button.hpp"
#include "textInput.hpp"
#include "scene.hpp"

namespace gfx {

class GlobalUpdateFunction {
public:
    virtual void update() = 0;
};

void addAGlobalUpdateFunction(GlobalUpdateFunction* global_update_function);

void init(int window_width_, int window_height_, const std::string& window_title);
void quit();

void loadFont(const unsigned char* data);

void resetRenderTarget();

inline bool blur_enabled = true;
inline int fps_limit = 0;

};
