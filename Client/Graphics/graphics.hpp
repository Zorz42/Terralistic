#ifndef graphics_hpp
#define graphics_hpp

#include <string>
#include <vector>
#include "color/color.hpp"
#include "rect/rect.hpp"
#include "scene/scene.hpp"
#include "ui/ui.hpp"

namespace gfx {

void init(unsigned short window_width_, unsigned short window_height_), quit();
inline std::string resource_path;
void setWindowMinimumSize(unsigned short width, unsigned short height);
void loadFont(const std::string& path, unsigned char size);


struct scene {
    virtual ~scene() = default;
    
    virtual void init() {}
    virtual void update() {}
    virtual void render() {}
    virtual void stop() {}
    virtual void onKeyDown(key key_) {}
    virtual void onKeyUp(key key_) {}
    virtual void onMouseScroll(int distance) {}
    
    std::vector<textInput*> text_inputs;
    std::vector<sceneModule*> modules;
    
    void _onKeyDown(key key_);
    void _onKeyUp(key key_);
    bool disable_events = false;
};

void runScene(scene* x);
void returnFromScene();

void* loadImageFile(const std::string& path);
void* renderText(const std::string& text, color text_color);
void* createBlankTexture(unsigned short w, unsigned short h);

unsigned short getMouseX(), getMouseY(), getWindowWidth(), getWindowHeight();
float getDeltaTime();

void setRenderTarget(image& tex);
void resetRenderTarget();
bool colliding(rectShape a, rectShape b);
unsigned int getTicks();

void clearWindow();
void updateWindow();

void sleep(unsigned short ms);

}

#endif /* graphics_hpp */
