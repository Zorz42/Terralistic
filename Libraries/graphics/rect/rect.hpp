#ifndef rect_hpp
#define rect_hpp

#include <SFML/Graphics.hpp>
#include "theme.hpp"

namespace gfx {

enum ObjectType {TOP_LEFT, TOP, TOP_RIGHT, LEFT, CENTER, RIGHT, BOTTOM_LEFT, BOTTOM, BOTTOM_RIGHT};

class Color{
public:
    unsigned char r, g, b, a;
    Color(unsigned char r, unsigned char g, unsigned char b, unsigned char a=255);
    inline operator sf::Color() const { return {r, g, b, a}; }

};

class RectShape {
public:
    short x, y;
    unsigned short w, h;
    RectShape(short x = 0, short y = 0, unsigned short w = 0, unsigned short h = 0);
    void render(Color c, bool fill=true) const;
};

class _CenteredObject {
public:
    _CenteredObject(short x, short y, ObjectType orientation = TOP_LEFT);
    ObjectType orientation;
    RectShape getTranslatedRect() const;
    inline virtual unsigned short getWidth() const { return 0; };
    inline virtual unsigned short getHeight() const { return 0; };
    short getTranslatedX() const;
    short getTranslatedY() const;

    short x, y;
};

class Rect : public _CenteredObject {
    sf::RenderTexture* shadow_texture = nullptr;
    sf::RenderTexture* blur_texture = nullptr;
    short prev_x, prev_y;
    unsigned short prev_w, prev_h;
    void updateShadowTexture();
public:
    explicit Rect(short x = 0, short y = 0, unsigned short w = 0, unsigned short h = 0, Color c = GFX_DEFAULT_RECT_COLOR, ObjectType orientation = TOP_LEFT);
    inline unsigned short getWidth() const override { return w; };
    inline unsigned short getHeight() const override { return h; };
    unsigned short w, h;
    Color c;
    float blur_intensity = 0;
    unsigned char shadow_intensity = 0;
    float shadow_blur = GFX_DEFAULT_SHADOW_BLUR;
    void render(bool fill=true);
    ~Rect();
};

};

#endif
