#pragma once
#include "launcherModule.hpp"

class ModuleManager : LauncherModule{
    bool onKeyDown(gfx::Key key) override;
    void update(float frame_length) override {enabled = true;};
    void init() override;
public:
    ModuleManager();
    std::vector<SceneModule*> module_vector;
    void moduleConfig(std::string command);
};