//
//  graphics.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 08/03/2021.
//

#ifndef graphics_hpp
#define graphics_hpp

#include <string>

namespace gfx {

void init(unsigned short window_width_, unsigned short window_height_), quit();
inline std::string resource_path;
void setWindowMinimumSize(unsigned short width, unsigned short height);
void loadFont(const std::string& path, unsigned char size);

void runScenes();

struct scene {
    scene(void(*initFunction)(), void(*renderFunction)(), void(*stopFunction)()) : initFunction(initFunction), renderFunction(renderFunction), stopFunction(stopFunction) {}
    void(*initFunction)();
    void(*renderFunction)();
    void(*stopFunction)();
};

enum objectType {top_left, top, top_right, left, center, right, bottom_left, bottom, bottom_right};

struct color {
    color(unsigned char r, unsigned char g, unsigned char b) : r(r), g(g), b(b) {}
    unsigned char r, g, b, a=255;
};

struct rectShape {
    rectShape(short x, short y, unsigned short w, unsigned short h) : x(x), y(y), w(w), h(h) {}
    short x, y;
    unsigned short w, h;
};

struct _centeredObject : public rectShape {
    _centeredObject(short x, short y, unsigned short w, unsigned short h, objectType orientation=top_left) : rectShape(x, y, w, h), orientation(orientation) {}
    objectType orientation;
    gfx::rectShape getRect() const;
    unsigned char scale = 1;
};

struct rect : public rectShape {
    rect(short x, short y, unsigned short w, unsigned short h, color c) : rectShape(x, y, w, h), c(c) {}
    color c;
};

struct texture {
    void loadFromFile(const std::string& path, unsigned short* w=nullptr, unsigned short* h=nullptr);
    void loadFromText(const std::string& text, color text_color, unsigned short* w=nullptr, unsigned short* h=nullptr);
private:
    void* tex=nullptr;
    friend void render(const texture& tex, short x, short y, unsigned short w, unsigned short h);
};

struct image {
    void loadFromFile(const std::string& path);
    void loadFromText(const std::string& text, color text_color);
    unsigned char scale = 1;
private:
    unsigned short w, h;
    texture tex;
    friend void render(const image& img, short x, short y);
};

struct sprite : public _centeredObject {
    sprite() : _centeredObject(0, 0, 0, 0) {};
    void loadFromFile(const std::string& path);
    void loadFromText(const std::string& text, color text_color);
private:
    texture tex;
    friend void render(const sprite& spr);
};

void render(rectShape x, color c);
void render(rect x);
void render(const texture& tex, short x, short y, unsigned short w, unsigned short h);
void render(const image& img, short x, short y);
void render(const sprite& spr);

void switchScene(scene* x);
void returnFromScene();

unsigned short getMouseX(), getMouseY(), getWindowWidth(), getWindowHeight();

}

#endif /* graphics_hpp */
