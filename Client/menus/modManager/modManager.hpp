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
    bool enabled;
    
    using gfx::Rect::getWidth;
    using gfx::Rect::getHeight;
    using gfx::Rect::getX;
    using gfx::Rect::setX;
    using gfx::Rect::getY;
    using gfx::Rect::setY;
    using gfx::Rect::orientation;
    using gfx::Rect::getTranslatedX;
    using gfx::Rect::getTranslatedY;
    using gfx::Rect::smooth_factor;
};

class ModManager : public gfx::Scene {
    GuiMod* holding = nullptr;
    std::vector<GuiMod*> mods;
    unsigned short hold_x, hold_y;
    short holding_x = 0, holding_y = 0, holding_vel_x = 0, holding_vel_y = 0;
    gfx::Rect placeholder;
    gfx::Sprite enabled_text, disabled_text;
    gfx::Button back_button;
    
    void init() override;
    void onKeyDown(gfx::Key key) override;
    void render() override;
    void stop() override;
    BackgroundRect* background;
public:
    ModManager(BackgroundRect* background) : background(background) {}
};

#endif
