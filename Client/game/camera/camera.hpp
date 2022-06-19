#pragma once
#include "clientModule.hpp"
#include "graphics.hpp"

class Camera : public ClientModule {
    float x = 35200, y = 9600, target_x = 35200, target_y = 9600;
    void update(float frame_length) override;
    gfx::Timer timer;
public:
    void setX(int x);
    void setY(int y);
    int getX() const;
    int getY() const;
    int getTargetX() const;
    int getTargetY() const;
    
    void jumpToTarget();
    
    int getViewBeginX() const;
    int getViewEndX() const;
    int getViewBeginY() const;
    int getViewEndY() const;
};
