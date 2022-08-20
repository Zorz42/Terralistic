#pragma once
#include "rectShape.hpp"

namespace gfx {

class Orientation {
public:
    float x, y;
};

inline const Orientation TOP_LEFT =     {0 , 0 };
inline const Orientation TOP =          {.5, 0 };
inline const Orientation TOP_RIGHT =    {1 , 0 };
inline const Orientation LEFT =         {0 , .5};
inline const Orientation CENTER =       {.5, .5};
inline const Orientation RIGHT =        {1 , .5};
inline const Orientation BOTTOM_LEFT =  {0 , 1 };
inline const Orientation BOTTOM =       {.5, 1 };
inline const Orientation BOTTOM_RIGHT = {1 , 1 };

class Container {
public:
    Container(int x = 0, int y = 0, int w = 0, int h = 0, Orientation orientation = TOP_LEFT);
    Orientation orientation;
    int x, y, w, h;
    virtual RectShape getTranslatedRect() const;
    Container* parent_containter = nullptr;
};

};
