#include "pauseScreen.hpp"
#include "settingsMenu.hpp"
#include "modManager.hpp"

void PauseScreen::init() {
    resume_button.scale = 3;
    resume_button.loadFromText("Resume");
    resume_button.y = SPACING;

    settings_button.scale = 3;
    settings_button.loadFromText("Settings");
    settings_button.y = resume_button.y + resume_button.getHeight() + SPACING;
    
    mods_button.scale = 3;
    mods_button.loadFromText("Mods");
    mods_button.y = settings_button.y + settings_button.getHeight() + SPACING;
    
    quit_button.scale = 3;
    quit_button.loadFromText("Leave Game");
    quit_button.y = mods_button.y + mods_button.getHeight() + SPACING;
    
    back_rect.fill_color.a = TRANSPARENCY;
    back_rect.shadow_intensity = SHADOW_INTENSITY;
    back_rect.border_color = BORDER_COLOR;
    back_rect.border_color.a = TRANSPARENCY;
    back_rect.setX(-(quit_button.getWidth() + 2 * SPACING) - 200);
    back_rect.smooth_factor = 3;
    back_rect.blur_intensity = BLUR;
}

bool PauseScreen::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::ESCAPE) {
        returnToGame();
        return true;
    }
    else if(key == gfx::Key::MOUSE_LEFT) {
        if(resume_button.isHovered(getMouseX(), getMouseY())) {
            returnToGame();
            return true;
        } else if(settings_button.isHovered(getMouseX(), getMouseY())) {
            SettingsMenu settings(this);
            switchToScene(settings);
            return true;
        } else if(mods_button.isHovered(getMouseX(), getMouseY())) {
            ModManager mod_manager(this);
            switchToScene(mod_manager);
            return true;
        } else if(quit_button.isHovered(getMouseX(), getMouseY())) {
            exitToMenu();
            return true;
        }
    }
    return false;
}

void PauseScreen::render() {
    if(!returning_to_game)
        back_rect.setWidth(quit_button.getWidth() + 2 * SPACING);
    renderBackground();
    renderButtons();
}

void PauseScreen::renderBackground() {
    background->renderBack();
    back_rect.setHeight(gfx::getWindowHeight());
    fade_rect.setWidth(gfx::getWindowWidth());
    fade_rect.setHeight(gfx::getWindowHeight());
    fade_rect.fill_color.a = float(back_rect.getWidth() + back_rect.getX() + 200) / (float)(back_rect.getWidth() + 200) * 70;
    fade_rect.blur_intensity = float(back_rect.getWidth() + back_rect.getX() + 200) / (float)(back_rect.getWidth() + 200) * BLUR / 2;
    if(fade_rect.blur_intensity < 0.5f)
        fade_rect.blur_intensity = 0;
    fade_rect.render();
    back_rect.render();
}

void PauseScreen::renderButtons() {
    resume_button.x = back_rect.getX() + SPACING;
    quit_button.x = back_rect.getX() + SPACING;
    mods_button.x = back_rect.getX() + SPACING;
    settings_button.x = back_rect.getX() + SPACING;
    resume_button.render(getMouseX(), getMouseY());
    settings_button.render(getMouseX(), getMouseY());
    mods_button.render(getMouseX(), getMouseY());
    quit_button.render(getMouseX(), getMouseY());
    
    if(returning_to_game) {
        if(back_rect.getX() == -back_rect.getWidth() - 200)
            returnFromScene();
    } else
        back_rect.setX(0);
}

void PauseScreen::returnToGame() {
    returning_to_game = true;
    back_rect.setX(-back_rect.getWidth() - 200);
}

void PauseScreen::exitToMenu() {
    exited_to_menu = true;
    returnFromScene();
}

bool PauseScreen::hasExitedToMenu() const {
    return exited_to_menu;
}

void PauseScreen::renderBack() {
    renderBackground();
    back_rect.setX(gfx::getWindowWidth() / 2 - getBackWidth() / 2);
}

void PauseScreen::setBackWidth(unsigned short width) {
    back_rect.setWidth(width);
}

unsigned short PauseScreen::getBackWidth() {
    return back_rect.getWidth();
}
