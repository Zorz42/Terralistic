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

enum key {KEY_MOUSE_LEFT, KEY_MOUSE_RIGHT, KEY_MOUSE_MIDDLE, KEY_A, KEY_B, KEY_C, KEY_D, KEY_E, KEY_F, KEY_G, KEY_H, KEY_I, KEY_J, KEY_K, KEY_L, KEY_M, KEY_N, KEY_O, KEY_P, KEY_Q, KEY_R, KEY_S, KEY_T, KEY_U, KEY_V, KEY_W, KEY_X, KEY_Y, KEY_Z, KEY_SPACE, KEY_0, KEY_1, KEY_2, KEY_3, KEY_4, KEY_5, KEY_6, KEY_7, KEY_8, KEY_9, KEY_UNKNOWN};

struct scene {
    virtual ~scene() { stop(); }
    
    virtual void init() {}
    virtual void update() {}
    virtual void render() {}
    virtual void stop() {}
    virtual void onKeyDown(key key_) {}
    virtual void onKeyUp(key key_) {}
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
    virtual gfx::rectShape getRect() const;
    void setPos(short x_, short y_) { x = x_; y = y_; }
    unsigned char scale = 1;
};

struct rect : public rectShape {
    rect(short x, short y, unsigned short w, unsigned short h, color c) : rectShape(x, y, w, h), c(c) {}
    color c;
};

struct texture {
    virtual void setSurface(void* surface);
protected:
    void* tex=nullptr;
    friend void render(const texture& tex, short x, short y, unsigned short w, unsigned short h);
};

struct image : public texture {
    void setSurface(void* surface);
    unsigned char scale = 1;
    unsigned short getWidth() { return w; }
    unsigned short getHeight() { return h; }
protected:
    unsigned short w, h;
    friend void render(const image& img, short x, short y);
};

struct sprite : public _centeredObject, texture {
    sprite() : _centeredObject(0, 0, 0, 0) {};
    void setSurface(void* surface);
protected:
    friend void render(const sprite& spr);
};

struct button : public sprite {
    void setText(const std::string& text, color text_color);
    unsigned short margin = 20;
    rectShape getRect() const;
    color def_color = {0, 0, 0}, hover_color = {0, 0, 0};
    bool isHovered() const;
protected:
    using sprite::setSurface;
    using sprite::w;
    using sprite::h;
    friend void render(const button& b);
};

void render(rectShape x, color c);
void render(rect x);
void render(const texture& tex, short x, short y, unsigned short w, unsigned short h);
void render(const image& img, short x, short y);
void render(const sprite& spr);
void render(const button& b);

void switchScene(scene* x);
void returnFromScene();

void* loadImageFile(const std::string& path);
void* renderText(const std::string& text, color text_color);

unsigned short getMouseX(), getMouseY(), getWindowWidth(), getWindowHeight();

}

#endif /* graphics_hpp */
