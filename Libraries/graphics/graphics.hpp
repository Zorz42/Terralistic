#ifndef graphics_hpp
#define graphics_hpp

#include <string>
#include <vector>
#include "rect.hpp"
#include "scene.hpp"
#include "ui.hpp"

namespace gfx {

void init(unsigned short window_width_, unsigned short window_height_), quit();
void loadFont(const std::string& path, unsigned char size);

inline std::string resource_path;

void setMinimumWindowSize(unsigned short width, unsigned short height);
unsigned short getWindowWidth();
unsigned short getWindowHeight();

void setRenderTarget(Image& tex);
void resetRenderTarget();

unsigned int getTicks();
void sleep(unsigned short ms);
float getDeltaTime();

void clearWindow();
void updateWindow();

void returnFromScene();

void setScale(float scale);

void setWindowSize(unsigned short width, unsigned short height);
    
void drawVertices(const sf::VertexArray& array, const sf::Texture& texture);
void drawVertices(const sf::VertexArray& array);

};

#endif
