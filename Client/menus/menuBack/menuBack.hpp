#pragma once
#include "graphics.hpp"

class BackgroundRect {
public:
    virtual void renderBack() = 0;
    virtual void setBackWidth(int width) = 0;
    virtual int getBackWidth() = 0;
};

class MenuBack : public BackgroundRect {
    gfx::Texture background;
    gfx::Rect back_rect;
public:
    void init();
    void renderBack() override;
    void setBackWidth(int width) override { back_rect.setWidth(width); }
    int getBackWidth() override { return back_rect.getWidth(); }
};
