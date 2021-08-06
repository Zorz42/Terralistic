#ifndef menuBack_hpp
#define menuBack_hpp

#include "graphics.hpp"

class MenuBack {
    gfx::Image background;
    gfx::Rect back_rect;
public:
    void init();
    void render();
    void setWidth(unsigned short width) { back_rect.setWidth(width); }
    unsigned short getWidth() { return back_rect.getWidth(); }
};

#endif
