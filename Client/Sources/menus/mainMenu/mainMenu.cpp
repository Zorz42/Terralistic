#include "mainMenu.hpp"
#include "worldSelector.hpp"
#include "multiplayerSelector.hpp"

void MainMenu::init() {
    singleplayer_button.scale = 3;
    singleplayer_button.renderText("Singleplayer");
    singleplayer_button.y = short(-singleplayer_button.getTranslatedRect().h - 5);
    singleplayer_button.orientation = gfx::CENTER;

    multiplayer_button.scale = 3;
    multiplayer_button.renderText("Multiplayer");
    multiplayer_button.orientation = gfx::CENTER;

    exit_button.scale = 3;
    exit_button.renderText("Exit");
    exit_button.y = short(exit_button.getTranslatedRect().h + 5);
    exit_button.orientation = gfx::CENTER;
    
    title.renderText("Terralistic");
    title.scale = 4;
    title.orientation = gfx::TOP;
    title.y = 40;
    
    background.loadFromFile("background.png");
    
    back_rect.orientation = gfx::CENTER;
    back_rect.w = singleplayer_button.getWidth() + 100;
    back_rect.c.a = TRANSPARENCY;
    back_rect.blur_intensity = BLUR;
    back_rect.enableShadow();
    
#ifdef DEVELOPER_MODE
    debug_title.renderText("DEBUG MODE", GREY);
    debug_title.orientation = gfx::TOP;
    debug_title.scale = 2;
    debug_title.y = 10;
#endif
    
    version.renderText(
#include "version.hpp"
                                       , GREY);
    version.orientation = gfx::BOTTOM;
    version.scale = 2;
    version.y = -5;
}

void MainMenu::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT) {
        if(exit_button.isHovered())
            gfx::returnFromScene();
        else if(singleplayer_button.isHovered())
            WorldSelector().run();
        else if(multiplayer_button.isHovered())
            MultiplayerSelector().run();
    }
}

void MainMenu::render() {
    float scale = (float)gfx::getWindowHeight() / (float)background.getTextureHeight();
    int pos = gfx::getTicks() / 30 % int(background.getTextureWidth() * scale);
    background.render(scale, pos, 0);
    background.render(scale, pos - background.getTextureWidth() * scale, 0);
    back_rect.h = gfx::getWindowHeight();
    back_rect.render();
    title.render();
#ifdef DEVELOPER_MODE
    debug_title.render();
#endif
    version.render();
    singleplayer_button.render();
    multiplayer_button.render();
    exit_button.render();
}
