#ifndef menuBack_hpp
#define menuBack_hpp

#include "graphics.hpp"

class MenuBack {
    gfx::Image background;
    gfx::Rect back_rect;
public:
    void init();
    void render();
    inline void setWidth(unsigned short width) { back_rect.setWidth(width); }
    inline unsigned short getWidth() { return back_rect.getWidth(); }
};

#endif
