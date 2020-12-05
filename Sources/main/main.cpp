//
//  main.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/07/2020.
//

#include "singleWindowLibrary.hpp"
#include "gameLoop.hpp"
#include "startMenu.hpp"
#include "framerateRegulator.hpp"
#include "fileSystem.hpp"
#include "blockEngine.hpp"
#include "playerHandler.hpp"
#include "blockSelector.hpp"
#include "pauseScreen.hpp"

int swl_main() {
    swl::loadFont("pixel_font.ttf", 8);
    framerateRegulator::fps_limit = 60;
    fileSystem::setDataPath();
    blockEngine::init();
    playerHandler::init();
    blockSelector::init();
    pauseScreen::init();
    
    startMenu::main();
    return 0;
}
