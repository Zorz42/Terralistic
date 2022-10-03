#pragma once
#include "menuBack.hpp"
#include "settings.hpp"

class RenderSetting {
public:
    virtual void render(int y, int width, int mouse_x, int mouse_y, int mouse_vel, bool is_mouse_pressed) = 0;
    virtual int getHeight() = 0;
    virtual int getWidth() = 0;
    virtual void onMouseButtonUp(int x, int y, int mouse_vel) {}
    virtual void onMouseButtonDown(int x, int y, int mouse_vel) {}
    virtual ~RenderSetting() = default;
};

class RenderChoiceSetting : public RenderSetting {
    ChoiceSetting* const setting;
    std::vector<gfx::Button*> choice_buttons;
    gfx::Sprite choice_text;
    gfx::Rect select_rect;
    void onMouseButtonUp(int x, int y, int mouse_vel) override;
public:
    explicit RenderChoiceSetting(ChoiceSetting* setting);
    
    void render(int y, int width, int mouse_x, int mouse_y, int mouse_vel, bool is_mouse_pressed) override;
    int getHeight() override;
    int getWidth() override;
};

class RenderBooleanSetting : public RenderSetting {
    BooleanSetting* const setting;
    gfx::Button toggle_button;
    gfx::Sprite text;
    void onMouseButtonUp(int x, int y, int mouse_vel) override;
    void updateButtonText();
public:
    explicit RenderBooleanSetting(BooleanSetting* setting);
    
    void render(int y, int width, int mouse_x, int mouse_y, int mouse_vel, bool is_mouse_pressed) override;
    int getHeight() override;
    int getWidth() override;
};

class RenderSliderSetting : public RenderSetting {
    SliderSetting* const setting;
    std::vector<gfx::Button*> choice_buttons;
    gfx::Sprite choice_text, slider_text;
    gfx::Rect select_rect, slider_rect;
    bool holding_slider = false, slider_hovered = false;
    void onMouseButtonUp(int x, int y, int mouse_vel) override;
    void onMouseButtonDown(int x, int y, int mouse_vel) override;
    void updateSliderText();
public:
    explicit RenderSliderSetting(SliderSetting* setting);
    
    void render(int y, int width, int mouse_x, int mouse_y, int mouse_vel, bool is_mouse_pressed) override;
    int getHeight() override;
    int getWidth() override;
};

class SettingsMenu : public gfx::Scene {
    gfx::Button back_button;
    gfx::Rect bottom_rect;
    
    std::vector<RenderSetting*> render_settings;
    
    void init() override;
    void stop() override;
    bool onKeyUp(gfx::Key key) override;
    bool onKeyDown(gfx::Key key) override;
    void onMouseScroll(int distance) override;
    void render() override;
    BackgroundRect* background;
    Settings* settings;
    int required_width = 0, required_height = 0;
    int scroll_offset = 0;
public:
    explicit SettingsMenu(BackgroundRect* background, Settings* settings) : gfx::Scene("SettingsMenu"), background(background), settings(settings) {}
};
