//
//  graphics.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 08/03/2021.
//

#ifndef graphics_hpp
#define graphics_hpp

#include <string>
#include <vector>

namespace gfx {

void init(unsigned short window_width_, unsigned short window_height_), quit();
inline std::string resource_path;
void setWindowMinimumSize(unsigned short width, unsigned short height);
void loadFont(const std::string& path, unsigned char size);

void runScenes();

enum key {KEY_MOUSE_LEFT, KEY_MOUSE_RIGHT, KEY_MOUSE_MIDDLE, KEY_A, KEY_B, KEY_C, KEY_D, KEY_E, KEY_F, KEY_G, KEY_H, KEY_I, KEY_J, KEY_K, KEY_L, KEY_M, KEY_N, KEY_O, KEY_P, KEY_Q, KEY_R, KEY_S, KEY_T, KEY_U, KEY_V, KEY_W, KEY_X, KEY_Y, KEY_Z, KEY_SPACE, KEY_0, KEY_1, KEY_2, KEY_3, KEY_4, KEY_5, KEY_6, KEY_7, KEY_8, KEY_9, KEY_UNKNOWN};

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

struct _centeredObject {
    _centeredObject(short x, short y, objectType orientation=top_left) : orientation(orientation), x(x), y(y) {}
    objectType orientation;
    rectShape getTranslatedRect() const;
    virtual unsigned short getWidth() const { return 0; };
    virtual unsigned short getHeight() const { return 0; };
    short getTranslatedX() const;
    short getTranslatedY() const;
    short x, y;
};

struct rect : public _centeredObject {
    rect(short x, short y, unsigned short w, unsigned short h, color c, objectType orientation=top_left) : _centeredObject(x, y, orientation), w(w), h(h), c(c) {}
    unsigned short getWidth() const { return w; };
    unsigned short getHeight() const { return h; };
    unsigned short w, h;
    color c;
};

struct texture {
    void setTexture(void* surface);
    void* getTexture() { return tex; }
    ~texture();
    bool free_texture = true;
    unsigned short getTextureWidth() const;
    unsigned short getTextureHeight() const;
    unsigned char scale = 1;
protected:
    void freeTexture();
    void* tex=nullptr;
    friend void render(const texture& tex, short x, short y);
    friend void render(const texture& tex, short x, short y, rectShape src_rect);
};

struct sprite : public _centeredObject, texture {
    unsigned short getWidth() const { return getTextureWidth() * scale; }
    unsigned short getHeight() const { return getTextureHeight() * scale; }
    sprite() : _centeredObject(0, 0) {};
};

struct button : public sprite {
    unsigned short margin = 10;
    
    unsigned short getWidth() const;
    unsigned short getHeight() const;
    
    color def_color = {0, 0, 0}, hover_color = {100, 100, 100};
    bool isHovered() const;
};

struct textInput : public button {
    unsigned short getWidth() const;
    textInput() { margin = 3; }
    color border_color = {255, 255, 255}, text_color = {255, 255, 255};
    std::string getText() const { return text; }
    void setText(const std::string& text);
    bool active = false;
    char (*textProcessing)(char c);
    unsigned short width = 200;
protected:
    std::string text;
};

void render(rectShape x, color c, bool fill=true);
void render(rect x, bool fill=true);
void render(const texture& tex, short x, short y);
void render(const texture& tex, short x, short y, rectShape src_rect);
void render(const sprite& spr);
void render(const button& b);
void render(const textInput& b);

struct scene {
    virtual ~scene() { stop(); }
    
    virtual void init() {}
    virtual void update() {}
    virtual void render() {}
    virtual void stop() {}
    virtual void onKeyDown(key key_) {}
    virtual void onKeyUp(key key_) {}
    std::vector<textInput*> text_inputs;
};

void switchScene(scene* x);
void returnFromScene();

void* loadImageFile(const std::string& path);
void* renderText(const std::string& text, color text_color);

unsigned short getMouseX(), getMouseY(), getWindowWidth(), getWindowHeight();

}

#endif /* graphics_hpp */
