#pragma once
#include "clientModule.hpp"

class Camera : public ClientModule {
    float x = 0, y = 0, target_x = 0, target_y = 0;
    void update(float frame_length) override;
    
public:
    void setX(int x);
    void setY(int y);
    int getX();
    int getY();
    int getTargetX();
    int getTargetY();
    
    void jumpToTarget();
    
    int getViewBeginX();
    int getViewEndX();
    int getViewBeginY();
    int getViewEndY();
};
