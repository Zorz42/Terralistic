#pragma once
#include <vector>
#include "button.hpp"
#include "rect.hpp"
#include "theme.hpp"

namespace gfx {

class TextInput : public Button {
    std::string text;
    Rect back_rect;
    std::vector<Key> passthrough_keys = {};
public:
    void render(int mouse_x, int mouse_y);
    TextInput();

    std::string getText() const { return text; }
    int getWidth() const override;
    void setText(const std::string& text);
    std::vector<Key> getPassthroughKeys() const { return passthrough_keys; }
    void setPassthroughKeys(std::vector<Key> new_keys) { passthrough_keys = std::move(new_keys); };

    bool active = false, ignore_next_input = false;
    char (*textProcessing)(char c, int length) = nullptr;
    int width = GFX_DEFAULT_TEXT_INPUT_WIDTH;
    Color text_color = GFX_DEFAULT_TEXT_COLOR;
    void setBlurIntensity(float blur_intensity);
    void setBorderColor(Color color);
};

};
