#include "glfwAbstraction.hpp"
#include "blur.hpp"
#include "shadow.hpp"
#include "font.hpp"
#include "graphics.hpp"

void gfx::init(int window_width, int window_height, const std::string& window_title) {
    initGlfw(window_width, window_height, window_title);
    initBlur();
    initShadow();
}

void gfx::quit() {
    quitFont();
    quitShadow();
    quitGlfw();
}
