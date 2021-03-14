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

class test_scene : public gfx::scene {
    gfx::texture dirt;
    gfx::button button;
    void init() {
        dirt.setSurface(gfx::loadImageFile("texturePack/blocks/dirt.png"));
        
        button.setText("Play", {255, 255, 255});
        button.scale = 3;
        button.setPos(10, 10);
        button.hover_color = {100, 100, 100};
    }
    
    void onKeyDown(gfx::key key) {
        
    }
    
    void onKeyUp(gfx::key key) {
        
    }
    
    void render() {
        gfx::render(gfx::rect(10, 10, 400, 500, {255, 0, 0}));
        gfx::render(dirt, gfx::rectShape(10, 100, 16, 16), gfx::rectShape(0, 0, 8, 8));
        gfx::render(button);
    }
};

int main(int argc, char **argv) {
    // initialize graphics and set resource path, which is a part of file loading in graphics
    
    gfx::init(1000, 600);
    gfx::loadFont("pixel_font.ttf", 8);
    gfx::resource_path = fileSystem::getResourcePath(argv[0]);
    gfx::setWindowMinimumSize(gfx::getWindowWidth(), gfx::getWindowHeight());
    
    gfx::switchScene(new test_scene());
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
