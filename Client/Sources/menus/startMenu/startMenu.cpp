//
//  startMenu.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/07/2020.
//

#include "startMenu.hpp"
#include "worldSelector.hpp"
#include "multiplayerSelector.hpp"

// this is the main menu, which you see on the start of the app

void startMenu::init() {
    singleplayer_button.scale = 3;
    singleplayer_button.setTexture(gfx::renderText("Singleplayer", {255, 255, 255}));
    singleplayer_button.y = short(-singleplayer_button.getTranslatedRect().h - 5);
    singleplayer_button.orientation = gfx::center;

    multiplayer_button.scale = 3;
    multiplayer_button.setTexture(gfx::renderText("Multiplayer", {255, 255, 255}));
    multiplayer_button.orientation = gfx::center;

    exit_button.scale = 3;
    exit_button.setTexture(gfx::renderText("Exit", {255, 255, 255}));
    exit_button.y = short(exit_button.getTranslatedRect().h + 5);
    exit_button.orientation = gfx::center;
    
    title.setTexture(gfx::renderText("Terralistic", {255, 255, 255}));
    title.scale = 4;
    title.orientation = gfx::top;
    title.y = 40;
    
    background.setTexture(gfx::loadImageFile("texturePack/misc/background.png"));
    
    back_rect.c = {0, 0, 0};
    back_rect.orientation = gfx::center;
    back_rect.w = singleplayer_button.getWidth() + 100;
#ifdef DEVELOPER_MODE
    debug_title.setTexture(gfx::renderText("DEBUG MODE", {100, 100, 100}));
    debug_title.orientation = gfx::top;
    debug_title.scale = 2;
    debug_title.y = 10;
#endif
    
    version.setTexture(gfx::renderText(
#include "version.hpp"
                                       , {100, 100, 100}));
    version.orientation = gfx::bottom;
    version.scale = 2;
    version.y = -5;
}

void startMenu::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_MOUSE_LEFT) {
        if(exit_button.isHovered())
            gfx::returnFromScene();
        else if(singleplayer_button.isHovered())
            gfx::runScene(new worldSelector());
        else if(multiplayer_button.isHovered())
            gfx::runScene(new multiplayerSelector());
    }
}

void startMenu::render() {
    background.scale = (float)gfx::getWindowHeight() / (float)background.getTextureHeight();
    int pos = gfx::getTicks() / 30 % int(background.getTextureWidth() * background.scale);
    gfx::render(background, pos, 0);
    gfx::render(background, pos - background.getTextureWidth() * background.scale, 0);
    back_rect.h = gfx::getWindowHeight();
    gfx::render(back_rect);
    gfx::render(title);
#ifdef DEVELOPER_MODE
    gfx::render(debug_title);
#endif
    gfx::render(version);
    gfx::render(singleplayer_button);
    gfx::render(multiplayer_button);
    gfx::render(exit_button);
}
