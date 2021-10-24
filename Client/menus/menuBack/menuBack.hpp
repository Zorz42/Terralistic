#pragma once

#include "graphics.hpp"

class BackgroundRect {
public:
    virtual void renderBack() = 0;
    virtual void setBackWidth(unsigned short width) = 0;
    virtual unsigned short getBackWidth() = 0;
};

class MenuBack : public BackgroundRect {
    gfx::Texture background;
    gfx::Rect back_rect;
public:
    void init();
    void renderBack() override;
    void setBackWidth(unsigned short width) override { back_rect.setWidth(width); }
    unsigned short getBackWidth() override { return back_rect.getWidth(); }
};
