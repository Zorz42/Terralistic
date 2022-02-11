#pragma once
#include "menuBack.hpp"
#include "settings.hpp"

class PauseScreen : public gfx::Scene, public BackgroundRect {
    gfx::Button resume_button, settings_button, mods_button, quit_button;
    gfx::Rect back_rect;
    gfx::Rect fade_rect;
    Settings* settings;
    void returnToGame();
    void exitToMenu();
    bool returning_to_game = false, exited_to_menu = false;
    void renderBackground();
    void renderButtons();
    int back_width;
    
    void init() override;
    bool onKeyDown(gfx::Key key) override;
    bool onKeyUp(gfx::Key key) override;
    void render() override;
    BackgroundRect* background;
public:
    explicit PauseScreen(BackgroundRect* background, Settings* settings) : background(background), settings(settings) {}
    bool hasExitedToMenu() const;
    
    void renderBack() override;
    void setBackWidth(int width) override;
    int getBackWidth() override;
};
