#ifndef graphics_hpp
#define graphics_hpp

#include <string>
#include <vector>
#include "color.hpp"
#include "rect.hpp"
#include "scene.hpp"
#include "ui.hpp"

namespace gfx {

void init(unsigned short window_width_, unsigned short window_height_), quit();
inline std::string resource_path;
void setWindowMinimumSize(unsigned short width, unsigned short height);
void loadFont(const std::string& path, unsigned char size);

void* loadImageFile(const std::string& path);
void* renderText(const std::string& text, Color text_color);
void* createBlankTexture(unsigned short w, unsigned short h);

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
