#pragma once
#include "../color/color.hpp"

enum objectType { TOP_LEFT, TOP, TOP_RIGHT, LEFT, CENTER, RIGHT, BOTTOM_LEFT, BOTTOM, BOTTOM_RIGHT };
class RectShape {
public:
    short x, y;
    unsigned short w, h;
    RectShape(short x = 0, short y = 0, unsigned short w = 0, unsigned short h = 0);
    void render(Color c, bool fill = true);
    void render(const Image& tex);
    void render(const Image& tex, float scale, short x, short y);
};

class _CenteredObject {
public:
    _CenteredObject(short x, short y, objectType orientation = TOP_LEFT);
    objectType orientation;
    [[nodiscard]] rectShape getTranslatedRect() const;
    [[nodiscard]] inline virtual unsigned short getWidth() const { return 0; };
    [[nodiscard]] inline virtual unsigned short getHeight() const { return 0; };
    [[nodiscard]] short getTranslatedX() const;
    [[nodiscard]] short getTranslatedY() const;

    short x, y;
};


class Rect : _CenteredObject {
public:
    explicit Rect(short x = 0, short y = 0, unsigned short w = 0, unsigned short h = 0, Color c = { 255, 255, 255 }, objectType orientation = TOP_LEFT);
    [[nodiscard]] inline unsigned short getWidth() const override { return w; };
    [[nodiscard]] inline unsigned short getHeight() const override { return h; };
    unsigned short w, h;
    Color c;

    void render(bool fill = true);

};