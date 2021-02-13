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

// this is the hearth of the program.

int main([[maybe_unused]] int argc, char **argv) {
    // initialize singleWindowLibrary and set resource path, which is a part of file loading in singleWindowLibrary
    swl::init();
    framerateRegulator::fps_limit = 60;
    swl::loadFont("pixel_font.ttf", 8);
    swl::resourcePath = fileSystem::getResourcePath(argv[0]);
    fileSystem::setDataPath();
    
    // initialize all modules, that need to be. Its mostly texture loading, rendering and preparing shapes, like defining rectangle size.
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
    
    // go to start menu
    startMenu::main();
    
    // exit
    swl::quit();
    return 0;
}
