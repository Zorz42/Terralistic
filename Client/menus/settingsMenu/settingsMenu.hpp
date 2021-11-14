#pragma once
#include "menuBack.hpp"
#include "settings.hpp"

class RenderSetting {
public:
    virtual void render(int y, int width, int mouse_x, int mouse_y) = 0;
    virtual int getHeight() = 0;
    virtual int getWidth() = 0;
    virtual void onMouseButtonDown(int x, int y) = 0;
    virtual ~RenderSetting() {}
};

class RenderChoiceSetting : public RenderSetting {
    ChoiceSetting* const setting;
    std::vector<gfx::Button*> choice_buttons;
    gfx::Sprite choice_text;
    gfx::Rect select_rect;
    void onMouseButtonDown(int x, int y) override;
public:
    RenderChoiceSetting(ChoiceSetting* setting);
    
    void render(int y, int width, int mouse_x, int mouse_y) override;
    int getHeight() override;
    int getWidth() override;
};

class RenderBooleanSetting : public RenderSetting {
    BooleanSetting* const setting;
    gfx::Button toggle_button;
    gfx::Sprite text;
    void onMouseButtonDown(int x, int y) override;
    void updateButtonText();
public:
    RenderBooleanSetting(BooleanSetting* setting);
    
    void render(int y, int width, int mouse_x, int mouse_y) override;
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
