#ifndef settings_hpp
#define settings_hpp

#include "graphics.hpp"
#include "menuBack.hpp"

class Settings : public gfx::Scene {
    gfx::Button back_button;
    
    void init() override;
    void render() override;
    void onKeyDown(gfx::Key key) override;
    BackgroundRect* background;
public:
    Settings(BackgroundRect* background) : background(background) {}
};

#endif
