#ifndef modManager_hpp
#define modManager_hpp

#include "graphics.hpp"
#include "menuBack.hpp"

class GuiMod : private gfx::Rect {
    std::string name;
    gfx::Image text;
public:
    GuiMod(const std::string& name);
    void render();
    const std::string& getName() { return name; }
    bool hoversPoint(unsigned short x, unsigned short y);
    
    using gfx::Rect::getWidth;
    using gfx::Rect::getHeight;
    using gfx::Rect::getX;
    using gfx::Rect::setX;
    using gfx::Rect::getY;
    using gfx::Rect::setY;
    using gfx::Rect::orientation;
    using gfx::Rect::getTranslatedX;
    using gfx::Rect::getTranslatedY;
};

class ModManager : public gfx::Scene {
    GuiMod* holding = nullptr;
    std::vector<GuiMod*> active_mods;
    std::vector<GuiMod*> inactive_mods;
    unsigned short hold_x, hold_y;
    gfx::Rect placeholder;
    
    void init() override;
    void onKeyDown(gfx::Key key) override;
    void render() override;
    void stop() override;
    BackgroundRect* background;
public:
    ModManager(BackgroundRect* background) : background(background) {}
};

#endif
