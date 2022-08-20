#pragma once
#include <vector>
#include "button.hpp"
#include "rect.hpp"
#include "theme.hpp"

namespace gfx {

class TextInput : public Button {
    bool cursor_end_active = false;
    int cursor[2] = {0, 0};
    std::string text;
    Rect back_rect;
    std::vector<Key> passthrough_keys = {};
public:
    void render(int mouse_x, int mouse_y, int mouse_vel);
    TextInput();

    std::string getText() const { return text; }
    int getWidth() const;
    
    void eraseSelected();
    int findLeftMove(int curr_pos, bool is_ctrl_pressed);
    int findRightMove(int curr_pos, bool is_ctrl_pressed);
    void setText(const std::string& text);
    void setCursor(int begin, int end) { cursor[0] = begin; cursor[1] = end; }
    void setCursor(int pos) { cursor[0] = pos; cursor[1] = pos; }
    void setCursorEndActive(bool active_) { cursor_end_active = active_; }
    bool getCursorEndActive() { return cursor_end_active; }
    int getCursorBegin() { return cursor[0]; }
    int getCursorEnd() { return cursor[1]; }
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
