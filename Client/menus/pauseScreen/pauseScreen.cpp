#include "pauseScreen.hpp"
#include "settingsMenu.hpp"
#include "modManager.hpp"

void PauseScreen::init() {
    resume_button.scale = 3;
    resume_button.loadFromSurface(gfx::textToSurface("Resume"));
    resume_button.y = SPACING;

    settings_button.scale = 3;
    settings_button.loadFromSurface(gfx::textToSurface("Settings"));
    settings_button.y = resume_button.y + resume_button.getHeight() + SPACING;
    
    mods_button.scale = 3;
    mods_button.loadFromSurface(gfx::textToSurface("Mods"));
    mods_button.y = settings_button.y + settings_button.getHeight() + SPACING;
    
    quit_button.scale = 3;
    quit_button.loadFromSurface(gfx::textToSurface("Leave Game"));
    quit_button.y = mods_button.y + mods_button.getHeight() + SPACING;
    
    back_rect.fill_color.a = TRANSPARENCY;
    back_rect.shadow_intensity = SHADOW_INTENSITY;
    back_rect.border_color = BORDER_COLOR;
    back_rect.border_color.a = TRANSPARENCY;
    back_rect.setX(-(quit_button.getWidth() + 2 * SPACING) - 200);
    back_rect.smooth_factor = 3;
    back_rect.blur_radius = BLUR;
}

bool PauseScreen::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::ESCAPE) {
        returnToGame();
        return true;
    }
    return false;
}

bool PauseScreen::onKeyUp(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT) {
        if(resume_button.isHovered(getMouseX(), getMouseY())) {
            returnToGame();
            return true;
        } else if(settings_button.isHovered(getMouseX(), getMouseY())) {
            SettingsMenu settings_menu(this, settings);
            settings_menu.run();
            return true;
        } else if(mods_button.isHovered(getMouseX(), getMouseY())) {
            ModManager mod_manager(this);
            mod_manager.run();
            if(mod_manager.changed_mods)
                changed_mods = true;
            return true;
        } else if(quit_button.isHovered(getMouseX(), getMouseY())) {
            exitToMenu();
            return true;
        }
    }
    return false;
}

void PauseScreen::render() {
    back_width = quit_button.getWidth() + 2 * SPACING;
    back_rect.setWidth(back_width);
    
    renderBackground();
    renderButtons();
}

void PauseScreen::renderBackground() {
    background->renderBack();
    if(back_rect.getHeight() != gfx::getWindowHeight()) {
        back_rect.setHeight(gfx::getWindowHeight());
        back_rect.jumpToTarget();
    }
    fade_rect.setWidth(gfx::getWindowWidth());
    fade_rect.setHeight(gfx::getWindowHeight());
    int back_x = std::min(back_rect.getX(), 0);
    fade_rect.fill_color.a = float(back_width + back_x + 200) / (float)(back_width + 200) * 70;
    fade_rect.blur_radius = float(back_width + back_x + 200) / (float)(back_width + 200) * BLUR / 2;
    if(fade_rect.blur_radius < 0.5f)
        fade_rect.blur_radius = 0;
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
        if(back_rect.getX() == -back_width - 200)
            returnFromScene();
    } else
        back_rect.setX(0);
}

void PauseScreen::returnToGame() {
    returning_to_game = true;
    back_rect.setX(-back_width - 200);
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

void PauseScreen::setBackWidth(int width) {
    back_rect.setWidth(width);
}

int PauseScreen::getBackWidth() {
    return back_rect.getWidth();
}
