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

#include <Graphics/graphics.hpp>

gfx::sprite dirt;

gfx::scene test_scene([]{
    dirt.loadFromFile("texturePack/blocks/dirt.png");
    dirt.scale = 2;
    dirt.x = 10;
    dirt.y = 10;
}, []{
    gfx::render(gfx::rect(10, 10, 400, 500, {255, 0, 0}));
    gfx::render(dirt);
}, []{
    
});

int main(int argc, char **argv) {
    // initialize graphics and set resource path, which is a part of file loading in graphics
    
    gfx::init(1000, 600);
    gfx::loadFont("pixel_font.ttf", 8);
    gfx::resource_path = fileSystem::getResourcePath(argv[0]);
    gfx::setWindowMinimumSize(gfx::getWindowWidth(), gfx::getWindowHeight());
    
    gfx::switchScene(&test_scene);
    gfx::runScenes();
    
    gfx::quit();
    /*swl::init();
    swl::loadFont("pixel_font.ttf", 8);
    swl::resourcePath = fileSystem::getResourcePath(argv[0]);
    swl::setWindowMinimumSize(swl::window_width, swl::window_height);
    
    init::initModules();
    
    // go to start menu
    startMenu::main();
    
    // exit
    swl::quit();*/
    return 0;
}
