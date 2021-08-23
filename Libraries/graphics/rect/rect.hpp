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
    operator sf::Color() const { return {r, g, b, a}; }

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
    virtual unsigned short getWidth() const { return 0; };
    virtual unsigned short getHeight() const { return 0; };
    short getTranslatedX(unsigned short width=0) const;
    short getTranslatedY(unsigned short height=0) const;

    short x, y;
};

class Rect : public _CenteredObject {
    sf::RenderTexture* shadow_texture = nullptr;
    sf::RenderTexture* blur_texture = nullptr;
    unsigned short prev_w, prev_h;
    unsigned char prev_shadow_intensity = 0;
    float prev_shadow_blur = GFX_DEFAULT_SHADOW_BLUR;
    void updateShadowTexture();
    using _CenteredObject::x;
    using _CenteredObject::y;
    unsigned short width, height;
    
    short target_x = 0, target_y = 0;
    unsigned short target_width = 0, target_height = 0;
    
    bool first_time = true;
    
    void updateBlurTextureSize();
    
public:
    explicit Rect(short x = 0, short y = 0, unsigned short w = 0, unsigned short h = 0, Color c = GFX_DEFAULT_RECT_COLOR, ObjectType orientation = TOP_LEFT);
    
    unsigned short getWidth() const override;
    void setWidth(unsigned short width_);
    
    unsigned short getHeight() const override;
    void setHeight(unsigned short height_);
    
    short getX() const;
    void setX(short x_);
    
    short getY() const;
    void setY(short y_);
    
    unsigned short smooth_factor = 1;
    Color c;
    float blur_intensity = 0;
    unsigned char shadow_intensity = 0;
    float shadow_blur = GFX_DEFAULT_SHADOW_BLUR;
    void render(bool fill=true);
    ~Rect();
    Color border_color = {0, 0, 0, 0};
};

};

#endif
