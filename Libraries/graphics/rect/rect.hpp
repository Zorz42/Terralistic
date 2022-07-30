#pragma once
#include "centeredObject.hpp"
#include "timer.hpp"

namespace gfx {

class Rect : public _CenteredObject {
    using _CenteredObject::x;
    using _CenteredObject::y;
    float width = 1, height = 1;
    
    int target_x = 0, target_y = 0, target_width = 1, target_height = 1;
    Timer approach_timer;
    int ms_counter = 0;
    
    bool first_time = true;
public:
    int getWidth() const override;
    void setWidth(int width_);
    
    int getHeight() const override;
    void setHeight(int height_);
    
    int getX() const;
    void setX(int x_);
    
    int getY() const;
    void setY(int y_);
    
    int getTargetX() const;
    int getTargetY() const;
    
    void jumpToTarget();
    
    int smooth_factor = 1;
    float blur_radius = 0;
    unsigned char shadow_intensity = 0;
    
    Color fill_color = {0, 0, 0, 0};
    Color border_color = {0, 0, 0, 0};
    
    void render();
};

};
