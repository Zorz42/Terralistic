#pragma once

namespace gfx {

class _CenteredObject {
public:
    explicit _CenteredObject(int x = 0, int y = 0, Orientation orientation = TOP_LEFT);
    Orientation orientation;
    RectShape getTranslatedRect() const;
    virtual int getWidth() const { return 0; };
    virtual int getHeight() const { return 0; };
    int getTranslatedX() const;
    int getTranslatedY() const;

    int x, y;
};

};
