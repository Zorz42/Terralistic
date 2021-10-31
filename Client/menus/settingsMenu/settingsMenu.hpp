#pragma once

#include "graphics.hpp"
#include "menuBack.hpp"
#include "settings.hpp"

class SettingsMenu : public gfx::Scene {
    gfx::Button back_button;
    
    void init() override;
    bool onKeyDown(gfx::Key key) override;
    void render() override;
    BackgroundRect* background;
    
public:
    explicit SettingsMenu(BackgroundRect* background/*, Settings* settings*/) : background(background) {}
};
