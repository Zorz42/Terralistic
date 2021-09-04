#ifndef modManager_hpp
#define modManager_hpp

#include "graphics.hpp"
#include "menuBack.hpp"

class GuiMod : private gfx::Button {
    std::string name;
public:
    GuiMod(const std::string& name);
    const std::string& getName() { return name; }
    
    using gfx::Button::render;
    using gfx::Button::getWidth;
    using gfx::Button::getHeight;
    using gfx::Button::x;
    using gfx::Button::y;
    using gfx::Button::orientation;
    using gfx::Button::getTranslatedX;
    using gfx::Button::getTranslatedY;
};

class ModManager : public gfx::Scene {
    void init() override;
    void onKeyDown(gfx::Key key) override;
    void render() override;
    void stop() override;
    BackgroundRect* background;
    std::vector<GuiMod*> active_mods;
    std::vector<GuiMod*> inactive_mods;
public:
    ModManager(BackgroundRect* background) : background(background) {}
};

#endif
