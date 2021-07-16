#pragma once

namespace gfx {

enum ObjectType {TOP_LEFT, TOP, TOP_RIGHT, LEFT, CENTER, RIGHT, BOTTOM_LEFT, BOTTOM, BOTTOM_RIGHT};

class RectShape {
public:
    short x, y;
    unsigned short w, h;
    RectShape(short x = 0, short y = 0, unsigned short w = 0, unsigned short h = 0);
    void render(Color c, bool fill=true);
};

class Color {
public:
    Color(unsigned char r, unsigned char g, unsigned char b, unsigned char a = 255);
    unsigned char r, g, b, a;
};

class _CenteredObject {
public:
    _CenteredObject(short x, short y, ObjectType orientation = TOP_LEFT);
    ObjectType orientation;
    [[nodiscard]] RectShape getTranslatedRect() const;
    [[nodiscard]] inline virtual unsigned short getWidth() const { return 0; };
    [[nodiscard]] inline virtual unsigned short getHeight() const { return 0; };
    [[nodiscard]] short getTranslatedX() const;
    [[nodiscard]] short getTranslatedY() const;

    short x, y;
};


class Rect : public _CenteredObject {
public:
    explicit Rect(short x = 0, short y = 0, unsigned short w = 0, unsigned short h = 0, Color c = { 255, 255, 255 }, ObjectType orientation = TOP_LEFT);
    [[nodiscard]] inline unsigned short getWidth() const override { return w; };
    [[nodiscard]] inline unsigned short getHeight() const override { return h; };
    unsigned short w, h;
    Color c;
    void render(bool fill=true) const;
};

};
