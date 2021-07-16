#ifndef graphics_hpp
#define graphics_hpp

#include <string>
#include <vector>
#include "rect.hpp"
#include "scene.hpp"
#include "ui.hpp"

namespace gfx {

void init(unsigned short window_width_, unsigned short window_height_), quit();
inline std::string resource_path;
void setWindowMinimumSize(unsigned short width, unsigned short height);
void loadFont(const std::string& path, unsigned char size);

unsigned short getMouseX(), getMouseY(), getWindowWidth(), getWindowHeight();
float getDeltaTime();

void setRenderTarget(Image& tex);
void resetRenderTarget();
bool colliding(RectShape a, RectShape b);
unsigned int getTicks();

void clearWindow();
void updateWindow();

void sleep(unsigned short ms);

void returnFromScene();

};

#endif /* graphics_hpp */
