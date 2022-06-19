#pragma once

namespace gfx {

enum class Key {MOUSE_LEFT, MOUSE_RIGHT, MOUSE_MIDDLE, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, NUM0, NUM1, NUM2, NUM3, NUM4, NUM5, NUM6, NUM7, NUM8, NUM9, SPACE, ESCAPE, ENTER, SHIFT, BACKSPACE, CTRL, ARROW_UP, ARROW_DOWN, ARROW_LEFT, ARROW_RIGHT, UNKNOWN};

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
