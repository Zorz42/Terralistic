//
//  main.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 19/02/2021.
//

#include "core.hpp"

#include "main.hpp"
#include "singleWindowLibrary.hpp"
#include "startMenu.hpp"

int main(int argc, char **argv) {
    // initialize graphics and set resource path, which is a part of file loading in graphics
    
    gfx::init(1000, 600);
    gfx::loadFont("pixel_font.ttf", 8);
    gfx::resource_path = fileSystem::getResourcePath(argv[0]);
    gfx::setWindowMinimumSize(gfx::getWindowWidth(), gfx::getWindowHeight());
    
    gfx::switchScene(new startMenu::scene());
    gfx::runScenes();
    
    gfx::quit();
    
    return 0;
}
