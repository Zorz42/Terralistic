#pragma once

#include "graphics.hpp"
#include "menuBack.hpp"
#include "settings.hpp"

class RenderSetting {
public:
    virtual void render(int y) = 0;
    virtual int getHeight() = 0;
    virtual int getWidth() = 0;
    virtual ~RenderSetting() {}
};

class RenderChoiceSetting : public RenderSetting {
    const ChoiceSetting* setting;
public:
    RenderChoiceSetting(ChoiceSetting* setting);
    
    void render(int y) override;
    int getHeight() override;
    int getWidth() override;
};

class SettingsMenu : public gfx::Scene {
    gfx::Button back_button;
    
    std::vector<RenderSetting*> render_settings;
    
    void init() override;
    void stop() override;
    bool onKeyDown(gfx::Key key) override;
    void render() override;
    BackgroundRect* background;
    Settings* settings;
    int required_width = 0;
public:
    explicit SettingsMenu(BackgroundRect* background, Settings* settings) : background(background), settings(settings) {}
};
