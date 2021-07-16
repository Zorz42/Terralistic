#pragma once

#include <string>
#include "rect.hpp"
#include <cassert>

namespace gfx {

class Image {
public:
    void render(RectShape rect) const;
    void render(float scale, short x, short y) const;
    void render(float scale, short x, short y, RectShape src_rect) const;
    [[nodiscard]] inline void* getTexture() const { return texture; }
    ~Image();
    bool free_texture = true, flipped = false;
    [[nodiscard]] unsigned short getTextureWidth() const;
    [[nodiscard]] unsigned short getTextureHeight() const;
    void clear();
    void setAlpha(unsigned char alpha);
    void createBlankImage(unsigned short width, unsigned short height);
    void renderText(const std::string& text, Color text_color);
    void loadFromFile(const std::string& path);
protected:
    void freeTexture();
    void* texture = nullptr;

    sf::Texture sfml_texture;
};


class Sprite : public _CenteredObject, public Image {
public:
    float scale = 1;
    [[nodiscard]] inline unsigned short getWidth() const override { return getTextureWidth() * scale; }
    [[nodiscard]] inline unsigned short getHeight() const override { return getTextureHeight() * scale; }
    Sprite();
    void render() const;

};

class Button : public Sprite {
public:
    unsigned short margin = 10;

    [[nodiscard]] unsigned short getWidth() const override;
    [[nodiscard]] unsigned short getHeight() const override;

    Color def_color = { 0, 0, 0 }, hover_color = { 100, 100, 100 };
    [[nodiscard]] bool isHovered() const;
    bool disabled = false;
    unsigned char hover_progress = 0;
    void render();
};

class TextInput : public Button {
public:
    void render() const;
    TextInput();

    [[nodiscard]] inline std::string getText() const { return text; }
    [[nodiscard]] unsigned short getWidth() const override;
    void setText(const std::string& text);

    bool active = false, ignore_one_input = false;
    char (*textProcessing)(char c, int length) = nullptr;
    unsigned short width = 200;
    Color border_color = { 255, 255, 255 }, text_color = { 255, 255, 255 };
    unsigned char cut_length;
protected:
    std::string text;
};

};
