#pragma once
#include "container.hpp"
#include "timer.hpp"

namespace gfx {

class Rect : public Container {
    float render_x = 0, render_y = 0, render_w = 0, render_h = 0;
    Timer approach_timer;
    int ms_counter = 0;
    bool first_time = true;
public:
    int smooth_factor = 1;
    float blur_radius = 0;
    unsigned char shadow_intensity = 0;
    
    Color fill_color = {0, 0, 0, 0};
    Color border_color = {0, 0, 0, 0};
    
    void jumpToTarget();
    void render();
    
    RectShape getTranslatedRect() const override;
};

};
