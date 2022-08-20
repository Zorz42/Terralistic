#pragma once
#include "graphics.hpp"

class BackgroundRect {
public:
    virtual void renderBack() = 0;
    virtual void setBackWidth(int width) = 0;
    virtual int getBackWidth() = 0;
    virtual gfx::Container* getBackContainer() = 0;
};

class MenuBack : public BackgroundRect {
    gfx::Texture background;
    gfx::Rect back_rect;
    gfx::Timer timer;
public:
    void init();
    void renderBack() override;
    void setBackWidth(int width) override { back_rect.w = width; }
    int getBackWidth() override { return back_rect.w; }
    gfx::Container* getBackContainer() override { return &back_rect; }
};
