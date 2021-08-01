#ifndef ui_hpp
#define ui_hpp

#include <cassert>
#include <string>
#include "rect.hpp"
#include "theme.hpp"

namespace gfx {

class Image {
public:
    void render(float scale, short x, short y, bool flipped=false) const;
    void render(float scale, short x, short y, RectShape src_rect, bool flipped=false) const;
    ~Image();
    bool free_texture = true;
    unsigned short getTextureWidth() const;
    unsigned short getTextureHeight() const;
    void clear();
    void createBlankImage(unsigned short width, unsigned short height);
    void renderText(const std::string& text, Color text_color=GFX_DEFAULT_TEXT_COLOR);
    void loadFromFile(const std::string& path);
    void setColor(Color color_);
    inline sf::RenderTexture* getSfmlTexture() { return sfml_render_texture; }
protected:
    void freeTexture();
    sf::RenderTexture *sfml_render_texture = nullptr;
    Color color{255, 255, 255};
};


class Sprite : public _CenteredObject, public Image {
public:
    bool flipped = false;
    float scale = 1;
    inline unsigned short getWidth() const override { return getTextureWidth() * scale; }
    inline unsigned short getHeight() const override { return getTextureHeight() * scale; }
    Sprite();
    void render() const;

};

class Button : public Sprite {
public:
    unsigned short margin = 10;

    unsigned short getWidth() const override;
    unsigned short getHeight() const override;

    Color def_color = GFX_DEFAULT_BUTTON_COLOR, hover_color = GFX_DEFAULT_HOVERED_BUTTON_COLOR;
    bool isHovered() const;
    bool disabled = false;
    unsigned char hover_progress = 0;
    void render();
};

class TextInput : public Button {
    std::string text;
    Rect back_rect;
public:
    void render();
    TextInput();

    inline std::string getText() const { return text; }
    unsigned short getWidth() const override;
    void setText(const std::string& text);

    bool active = false, ignore_one_input = false;
    char (*textProcessing)(char c, int length) = nullptr;
    unsigned short width = 200;
    Color border_color = GFX_DEFAULT_TEXT_INPUT_BORDER_COLOR, text_color = GFX_DEFAULT_TEXT_COLOR;
    unsigned char cut_length;
    void setBlurIntensity(float blur_intensity);
    void setBorderColor(Color color);
};

};

#endif
