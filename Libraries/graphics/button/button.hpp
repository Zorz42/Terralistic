#pragma once
#include "glfwAbstraction.hpp"
#include "sprite.hpp"
#include "timer.hpp"
#include "theme.hpp"

namespace gfx {

class Button : public Texture, public Container {
    gfx::Timer timer;
    int timer_counter = 0;
    float scale = 1;
    int margin = GFX_DEFAULT_BUTTON_MARGIN;
    
    using gfx::Container::w;
    using gfx::Container::h;
    
    void updateSize();
public:
    Color def_color = GFX_DEFAULT_BUTTON_COLOR, def_border_color = GFX_DEFAULT_BUTTON_BORDER_COLOR, hover_color = GFX_DEFAULT_HOVERED_BUTTON_COLOR, border_hover_color = GFX_DEFAULT_HOVERED_BUTTON_BORDER_COLOR;
    bool isHovered(int mouse_x, int mouse_y) const;
    bool disabled = false;
    float hover_progress = 0;
    void render(int mouse_x, int mouse_y, bool button_pressed);
    
    void loadFromSurface(const Surface& surface);
    
    void setScale(float scale);
    float getScale() const;
    
    void setMargin(int margin);
    int getMargin() const;
    
    int getWidth() const;
    int getHeight() const;
};

};
