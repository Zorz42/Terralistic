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

enum key {KEY_MOUSE_LEFT, KEY_MOUSE_RIGHT, KEY_MOUSE_MIDDLE, KEY_A, KEY_B, KEY_C, KEY_D, KEY_E, KEY_F, KEY_G, KEY_H, KEY_I, KEY_J, KEY_K, KEY_L, KEY_M, KEY_N, KEY_O, KEY_P, KEY_Q, KEY_R, KEY_S, KEY_T, KEY_U, KEY_V, KEY_W, KEY_X, KEY_Y, KEY_Z, KEY_0, KEY_1, KEY_2, KEY_3, KEY_4, KEY_5, KEY_6, KEY_7, KEY_8, KEY_9, KEY_SPACE, KEY_ESCAPE, KEY_ENTER, KEY_SHIFT, KEY_UNKNOWN};

enum objectType {top_left, top, top_right, left, center, right, bottom_left, bottom, bottom_right};

struct color {
    color(unsigned char r, unsigned char g, unsigned char b, unsigned char a=255) : r(r), g(g), b(b), a(a) {}
    unsigned char r, g, b, a;
};

struct rectShape {
    explicit rectShape(short x=0, short y=0, unsigned short w=0, unsigned short h=0) : x(x), y(y), w(w), h(h) {}
    short x, y;
    unsigned short w, h;
};

struct _centeredObject {
    _centeredObject(short x, short y, objectType orientation=top_left) : orientation(orientation), x(x), y(y) {}
    objectType orientation;
    [[nodiscard]] rectShape getTranslatedRect() const;
    [[nodiscard]] inline virtual unsigned short getWidth() const { return 0; };
    [[nodiscard]] inline virtual unsigned short getHeight() const { return 0; };
    [[nodiscard]] short getTranslatedX() const;
    [[nodiscard]] short getTranslatedY() const;
    short x, y;
};

struct rect : _centeredObject {
    explicit rect(short x=0, short y=0, unsigned short w=0, unsigned short h=0, color c={255, 255, 255}, objectType orientation=top_left) : _centeredObject(x, y, orientation), w(w), h(h), c(c) {}
    [[nodiscard]] inline unsigned short getWidth() const override { return w; };
    [[nodiscard]] inline unsigned short getHeight() const override { return h; };
    unsigned short w, h;
    color c;
};

struct image {
    void setTexture(void* texture_);
    [[nodiscard]] void* getTexture() const { return texture; }
    ~image();
    bool free_texture = true, flipped = false;
    [[nodiscard]] unsigned short getTextureWidth() const;
    [[nodiscard]] unsigned short getTextureHeight() const;
    float scale = 1;
    void clear();
    void setAlpha(unsigned char alpha);
protected:
    void freeTexture();
    void* texture=nullptr;
};

struct sprite : _centeredObject, image {
    [[nodiscard]] inline unsigned short getWidth() const override { return getTextureWidth() * scale; }
    [[nodiscard]] inline unsigned short getHeight() const override { return getTextureHeight() * scale; }
    sprite() : _centeredObject(0, 0) {};
};

struct button : sprite {
    unsigned short margin = 10;
    
    [[nodiscard]] unsigned short getWidth() const override;
    [[nodiscard]] unsigned short getHeight() const override;
    
    color def_color = {0, 0, 0}, hover_color = {100, 100, 100};
    [[nodiscard]] bool isHovered() const;
    bool disabled = false;
    unsigned char hover_progress = 0;
};

struct textInput : button {
    textInput();
    
    [[nodiscard]] inline std::string getText() const { return text; }
    [[nodiscard]] unsigned short getWidth() const override;
    void setText(const std::string& text);
    
    bool active = false, ignore_one_input = false;
    char (*textProcessing)(char c, int length) = nullptr;
    unsigned short width = 200;
    color border_color = {255, 255, 255}, text_color = {255, 255, 255};
    unsigned char cut_length;
protected:
    std::string text;
};

void render(rectShape x, color c, bool fill=true);
void render(const rect& x, bool fill=true);
void render(const image& tex, short x, short y);
void render(const image& tex, rectShape rect);
void render(const image& tex, short x, short y, rectShape src_rect);
void render(const sprite& spr);
void render(button& b);
void render(const textInput& b);

struct scene;

struct sceneModule {
    virtual ~sceneModule() = default;
    
    virtual void init() {}
    virtual void update() {}
    virtual void render() {}
    virtual void stop() {}
    virtual void onKeyDown(key key_) {}
    virtual void onKeyUp(key key_) {}
    
    std::vector<textInput*> text_inputs;
    bool disable_events = false;
};

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
