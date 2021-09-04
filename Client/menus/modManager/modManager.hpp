#ifndef modManager_hpp
#define modManager_hpp

#include "graphics.hpp"
#include "menuBack.hpp"

class ModManager : public gfx::Scene {
    void init() override;
    void onKeyDown(gfx::Key key) override;
    void render() override;
    BackgroundRect* background;
public:
    ModManager(BackgroundRect* background) : background(background) {}
};

#endif
