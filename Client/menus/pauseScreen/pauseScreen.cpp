#include <cmath>
#include "pauseScreen.hpp"
#include "settings.hpp"

void PauseScreen::init() {
    resume_button.scale = 3;
    resume_button.renderText("Resume");
    resume_button.y = SPACING;

    settings_button.scale = 3;
    settings_button.renderText("Settings");
    settings_button.y = resume_button.y + resume_button.getHeight() + SPACING;
    
    quit_button.scale = 3;
    quit_button.renderText("Leave Game");
    quit_button.y = settings_button.y + settings_button.getHeight() + SPACING;
    
    int width = quit_button.getWidth() + 2 * SPACING;
    back_rect.setWidth(width);
    back_rect.c.a = TRANSPARENCY;
    back_rect.shadow_intensity = SHADOW_INTENSITY;
    back_rect.border_color = BORDER_COLOR;
    back_rect.border_color.a = TRANSPARENCY;
    back_rect.setX(-width - 200);
    back_rect.smooth_factor = 3;
    back_rect.blur_intensity = BLUR;
}

void PauseScreen::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::ESCAPE)
        returnToGame();
    else if(key == gfx::Key::MOUSE_LEFT) {
        if(resume_button.isHovered(mouse_x, mouse_y))
            returnToGame();
        else if(settings_button.isHovered(mouse_x, mouse_y)) {
            Settings(this).run();
        } else if(quit_button.isHovered(mouse_x, mouse_y)) {
            exitToMenu();
        }
    }
}

void PauseScreen::render() {
    renderBackground();
    renderButtons();
}

void PauseScreen::renderBackground() {
    background->renderBack();
    back_rect.setHeight(gfx::getWindowHeight());
    fade_rect.setWidth(gfx::getWindowWidth());
    fade_rect.setHeight(gfx::getWindowHeight());
    fade_rect.c.a = float(back_rect.getWidth() + back_rect.getX() + 200) / (float)(back_rect.getWidth() + 200) * 70;
    fade_rect.blur_intensity = float(back_rect.getWidth() + back_rect.getX() + 200) / (float)(back_rect.getWidth() + 200) * BLUR / 2;
    if(fade_rect.blur_intensity < 0.5f)
        fade_rect.blur_intensity = 0;
    fade_rect.render();
    back_rect.render();
    
    if(returning_to_game) {
        if(back_rect.getX() == -back_rect.getWidth() - 200)
            gfx::returnFromScene();
    } else
        back_rect.setX(0);
}

void PauseScreen::renderButtons() {
    resume_button.x = back_rect.getX() + SPACING;
    quit_button.x = back_rect.getX() + SPACING;
    settings_button.x = back_rect.getX() + SPACING;
    resume_button.render(mouse_x, mouse_y);
    settings_button.render(mouse_x, mouse_y);
    quit_button.render(mouse_x, mouse_y);
}

void PauseScreen::returnToGame() {
    returning_to_game = true;
    back_rect.setX(-back_rect.getWidth() - 200);
}

void PauseScreen::exitToMenu() {
    exited_to_menu = true;
    gfx::returnFromScene();
}

bool PauseScreen::hasExitedToMenu() {
    return exited_to_menu;
}

void PauseScreen::renderBack() {
    renderBackground();
}
