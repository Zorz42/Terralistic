//
//  main.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/07/2020.
//

#include "singleWindowLibrary.hpp"
#include "startMenu.hpp"
#include "framerateRegulator.hpp"
#include "fileSystem.hpp"
#include "playerHandler.hpp"
#include "blockSelector.hpp"
#include "pauseScreen.hpp"
#include "blockEngine.hpp"
#include "worldSaver.hpp"
#include "worldSelector.hpp"
#include "worldCreator.hpp"
#include "itemEngine.hpp"
#include "multiplayerSelector.hpp"
#include "networkingModule.hpp"

int main([[maybe_unused]] int argc, char **argv) {
    swl::init();
    swl::loadFont("pixel_font.ttf", 8);
    framerateRegulator::fps_limit = 60;
    fileSystem::setDataPath(argv[0]);
    playerHandler::init();
    blockSelector::init();
    pauseScreen::init();
    blockEngine::init();
    worldSaver::init();
    worldSelector::init();
    worldCreator::init();
    itemEngine::init();
    multiplayerSelector::init();
    networking::init();
    swl::setWindowMinimumSize(swl::window_width, swl::window_height);
    startMenu::main();
    swl::quit();
    return 0;
}
