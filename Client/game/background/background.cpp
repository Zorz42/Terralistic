#include "background.hpp"

void Background::init() {
    background.loadFromFile(resource_pack->getFile("/misc/background.png"));
}

void Background::render() {
    float scale = (float)gfx::getWindowHeight() / background.getTextureHeight();
    int position_x = -(camera->getX() / 5) % int(background.getTextureWidth() * scale);
    for(int i = 0; i < gfx::getWindowWidth() / (background.getTextureWidth() * scale) + 2; i++)
        background.render(scale, position_x + i * background.getTextureWidth() * scale, 0);
}
