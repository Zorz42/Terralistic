//
//  main.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 19/02/2021.
//

#include "core.hpp"

#include "main.hpp"
#include "startMenu.hpp"

int main(int argc, char **argv) {
    // initialize graphics and set resource path, which is a part of file loading in graphics
    
    gfx::init(1000, 600);
    gfx::loadFont("pixel_font.ttf", 8);
    gfx::resource_path = fileSystem::getResourcePath(argv[0]);
    gfx::setWindowMinimumSize(gfx::getWindowWidth(), gfx::getWindowHeight());
    
    init::initModules();
    
    gfx::switchScene(new startMenu::scene());
    gfx::runScenes();
    
    gfx::quit();
    
    return 0;
}

/*#include <Graphics/graphics.hpp>

struct scene : public gfx::scene {
    gfx::sprite hello_text;
    void init() {
        hello_text.setTexture(gfx::renderText("hello", {255, 255, 255}));
        hello_text.orientation = gfx::center;
        hello_text.scale = 3;
    }
    
    void render() {
        gfx::render(hello_text);
    }
    
    void stop() {
        
    }
};

int main() {
    
    gfx::init(1000, 600);
    gfx::loadFont("pixel_font.ttf", 8);
    gfx::resource_path = "../Resources";
    gfx::setWindowMinimumSize(gfx::getWindowWidth(), gfx::getWindowHeight());
    
    gfx::switchScene(new scene());
    gfx::runScenes();
    
    gfx::quit();
    
    return 0;
}*/
