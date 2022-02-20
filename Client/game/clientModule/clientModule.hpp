#pragma once
#include "graphics.hpp"

class ClientModule : public gfx::SceneModule {
public:
    virtual void updateParallel(float frame_length) {} 
    virtual void postInit() {}
    virtual void loadTextures() {}
};
