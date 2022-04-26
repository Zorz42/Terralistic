#pragma once
#include "menuBack.hpp"

class GuiMod : private gfx::Rect {
    std::string name;
    gfx::Texture text;
public:
    explicit GuiMod(const std::string& name);
    void renderTile();
    const std::string& getName() { return name; }
    bool hoversPoint(int x, int y);
    bool enabled = true;
    
    using gfx::Rect::getWidth;
    using gfx::Rect::getHeight;
    using gfx::Rect::getX;
    using gfx::Rect::setX;
    using gfx::Rect::getY;
    using gfx::Rect::setY;
    using gfx::Rect::getTargetY;
    using gfx::Rect::orientation;
    using gfx::Rect::getTranslatedX;
    using gfx::Rect::getTranslatedY;
    using gfx::Rect::smooth_factor;
};

class ModManager : public gfx::Scene {
    GuiMod* holding = nullptr;
    std::vector<GuiMod*> mods;
    int hold_x = 0, hold_y = 0;
    int holding_x = 0, holding_y = 0, holding_vel_x = 0, holding_vel_y = 0;
    gfx::Rect placeholder;
    gfx::Sprite enabled_text, disabled_text;
    gfx::Button back_button;
    
    void init() override;
    bool onKeyDown(gfx::Key key) override;
    bool onKeyUp(gfx::Key key) override;
    void render() override;
    void stop() override;
    BackgroundRect* background;
public:
    bool changed_mods = false;
    explicit ModManager(BackgroundRect* background) : background(background) {}
};
