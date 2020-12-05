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

int swl_main() {
    swl::loadFont("pixel_font.ttf", 8);
    framerateRegulator::fps_limit = 60;
    fileSystem::setDataPath();
    
    if(!startMenu::main())
        return 0;
    return gameLoop::main();
}
