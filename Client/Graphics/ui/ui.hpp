#pragma once
#include "../rect/rect.hpp"
class Image {
public:
    void render(float scale, short x, short y);

    void setTexture(void* texture_);
    [[nodiscard]] inline void* getTexture() const { return texture; }
    ~Image();
    bool free_texture = true, flipped = false;
    [[nodiscard]] unsigned short getTextureWidth() const;
    [[nodiscard]] unsigned short getTextureHeight() const;
    void clear();
    void setAlpha(unsigned char alpha);
protected:
    void freeTexture();
    void* texture = nullptr;
};


class Sprite : _CenteredObject, Image {
public:
    float scale = 1;
    [[nodiscard]] inline unsigned short getWidth() const override { return getTextureWidth() * scale; }
    [[nodiscard]] inline unsigned short getHeight() const override { return getTextureHeight() * scale; }
    Sprite();
    void render();
};

class Button : Sprite {
public:
    unsigned short margin = 10;

    [[nodiscard]] unsigned short getWidth() const override;
    [[nodiscard]] unsigned short getHeight() const override;

    color def_color = { 0, 0, 0 }, hover_color = { 100, 100, 100 };
    [[nodiscard]] bool isHovered() const;
    bool disabled = false;
    unsigned char hover_progress = 0;
    void render();
};

class TextInput : Button {
public:
    void render();

    TextInput();

    [[nodiscard]] inline std::string getText() const { return text; }
    [[nodiscard]] unsigned short getWidth() const override;
    void setText(const std::string& text);

    bool active = false, ignore_one_input = false;
    char (*textProcessing)(char c, int length) = nullptr;
    unsigned short width = 200;
    color border_color = { 255, 255, 255 }, text_color = { 255, 255, 255 };
    unsigned char cut_length;
protected:
    std::string text;
};
