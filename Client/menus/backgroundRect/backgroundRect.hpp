#pragma once
#include "graphics.hpp"

class Background {
public:
    virtual void renderBack() = 0;
};

class BackgroundRect : public Background {
public:
    virtual void setBackWidth(int width) = 0;
    virtual int getBackWidth() = 0;
    virtual gfx::Container* getBackContainer() = 0;
};
