#pragma once
#include "color.hpp"

namespace gfx {

class RectShape {
public:
    RectShape(int x, int y, int w, int h);
    RectShape() = default;
    int x = 0, y = 0, w = 0, h = 0;
    void render(Color color) const;
    void renderOutline(Color color) const;
};

};
