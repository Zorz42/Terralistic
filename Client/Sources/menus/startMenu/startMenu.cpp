//
//  startMenu.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/07/2020.
//

#include "core.hpp"

#include "startMenu.hpp"
#include "singleWindowLibrary.hpp"
#include "UIKit.hpp"
#include "gameLoop.hpp"
#include "worldSelector.hpp"
#include "multiplayerSelector.hpp"
#include "main.hpp"

#undef main

// this is the main menu, which you see on the start of the app

gfx::button singleplayer_button, multiplayer_button, exit_button;

INIT_SCRIPT
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
INIT_SCRIPT_END

void startMenu::scene::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_MOUSE_LEFT) {
        if(exit_button.isHovered())
            gfx::returnFromScene();
        else if(singleplayer_button.isHovered())
            gfx::switchScene(new worldSelector::scene());
    }
}

void startMenu::scene::render() {
    gfx::render(singleplayer_button);
    gfx::render(multiplayer_button);
    gfx::render(exit_button);
}
