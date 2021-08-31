#ifndef pauseScreen_hpp
#define pauseScreen_hpp

#include "graphics.hpp"
#include "menuBack.hpp"

class PauseScreen : public gfx::Scene, public BackgroundRect {
    gfx::Button resume_button, settings_button, quit_button;
    gfx::Rect back_rect;
    gfx::Rect fade_rect;
    void returnToGame();
    void exitToMenu();
    bool returning_to_game = false, exited_to_menu = false;
    void renderBackground();
    void renderButtons();
    
    void init() override;
    void onKeyDown(gfx::Key key) override;
    void render() override;
    BackgroundRect* background;
public:
    PauseScreen(BackgroundRect* background) : background(background) {}
    bool hasExitedToMenu();
    
    void renderBack() override;
    void setBackWidth(unsigned short width) override;
    unsigned short getBackWidth() override;
};

#endif
