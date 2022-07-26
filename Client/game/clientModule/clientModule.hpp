#pragma once
#include "graphics.hpp"

class ClientModule : public gfx::SceneModule {
public:
    ClientModule(const std::string& module_name) : gfx::SceneModule(module_name) {}
    virtual void updateParallel(float frame_length) {}
    virtual void updatePerMs() {}
    virtual void postInit() {}
    virtual void loadTextures() {}
};
