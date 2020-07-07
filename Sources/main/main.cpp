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

int swl_main() {
    swl::loadFont("pixel_font.ttf", 8);
    framerateRegulator::fps_limit = 60;
    
    int result = startMenu::main();
    if(result == -1)
        return 1;
    if(!result)
        return 0;
    result = gameLoop::main();
    if(result)
        return result;
    return 0;
}
