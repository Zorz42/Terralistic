#ifndef menuBack_hpp
#define menuBack_hpp

#include "graphics.hpp"

class MenuBack {
    gfx::Image background;
    gfx::Rect back_rect;
    unsigned short width = 0;
public:
    void init();
    void render();
    inline void setWidth(unsigned short width_) { width = width_; }
    inline unsigned short getWidth() { return back_rect.getWidth(); }
};

#endif
