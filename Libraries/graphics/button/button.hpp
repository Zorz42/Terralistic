#pragma once

namespace gfx {

class Button : public Sprite {
    gfx::Timer timer;
public:
    int margin = GFX_DEFAULT_BUTTON_MARGIN;

    int getWidth() const override;
    int getHeight() const override;

    Color def_color = GFX_DEFAULT_BUTTON_COLOR, def_border_color = GFX_DEFAULT_BUTTON_BORDER_COLOR, hover_color = GFX_DEFAULT_HOVERED_BUTTON_COLOR, border_hover_color = GFX_DEFAULT_HOVERED_BUTTON_BORDER_COLOR;
    bool isHovered(int mouse_x, int mouse_y) const;
    bool disabled = false;
    float hover_progress = 0;
    void render(int mouse_x, int mouse_y);
};

};
