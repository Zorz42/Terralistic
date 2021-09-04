#ifndef settings_hpp
#define settings_hpp

#include "graphics.hpp"
#include "menuBack.hpp"

class Settings : public gfx::Scene {
    gfx::Button back_button, small_scale_button, normal_scale_button, large_scale_button, mods_button;
    gfx::Sprite scale_text;
    gfx::Rect scale_back_rect, scale_select_rect;
    
    void init() override;
    void onKeyDown(gfx::Key key) override;
    void render() override;
    BackgroundRect* background;
    
    void reloadSettings();
    void updateScaleRect();
public:
    Settings(BackgroundRect* background) : background(background) {}
};

void loadSettings();

#endif
