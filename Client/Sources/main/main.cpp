//
//  main.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 19/02/2021.
//

#include "core.hpp"

#include "singleWindowLibrary.hpp"
#include "startMenu.hpp"

int main(int argc, char **argv) {
    // initialize singleWindowLibrary and set resource path, which is a part of file loading in singleWindowLibrary

    swl::init();
    swl::resourcePath = fileSystem::getResourcePath(argv[0]);
    swl::loadFont("pixel_font.ttf", 8);
    swl::setWindowMinimumSize(swl::window_width, swl::window_height);
    
    init::initModules();
    
    // go to start menu
    startMenu::loop();
    
    // exit
    swl::quit();
    return 0;
}
