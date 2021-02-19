//
//  init.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 19/02/2021.
//

#include <vector>
#include "init.hpp"
#include "startMenu.hpp"
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
#include "singleWindowLibrary.hpp"

std::vector<init::initFunction>& getInitFunctions() {
    static std::vector<init::initFunction> init_functions;
    return init_functions;
}

init::registerInitFunction::registerInitFunction(initFunction function) {
    getInitFunctions().push_back(function);
}

int main(int argc, char **argv) {
    // initialize singleWindowLibrary and set resource path, which is a part of file loading in singleWindowLibrary
    swl::init();
    swl::loadFont("pixel_font.ttf", 8);
    swl::resourcePath = fileSystem::getResourcePath(argv[0]);
    fileSystem::setDataPath();
    
    // initialize all modules, that need to be. Its mostly texture loading, rendering and preparing shapes, like defining rectangle size.
    for(init::initFunction func : getInitFunctions())
        func();

    swl::setWindowMinimumSize(swl::window_width, swl::window_height);
    
    // go to start menu
    startMenu::main();
    
    // exit
    swl::quit();
    return 0;
}
