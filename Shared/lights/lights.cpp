#include "lights.hpp"

void Lights::create() {
    lights = new Light[blocks->getWidth() * blocks->getHeight()];
}

Lights::Light* Lights::getLight(unsigned short x, unsigned short y) {
    if(x >= blocks->getWidth() || y >= blocks->getHeight())
        throw LightOutOfBoundsException();
    return &lights[y * blocks->getWidth() + x];
}

void Lights::setLightLevel(unsigned short x, unsigned short y, unsigned char level) {
    if(level == 0)
        getLight(x, y)->light_source = false;
    getLight(x, y)->light_level = level;
}

void Lights::updateLight(unsigned short x, unsigned short y) {
    getLight(x, y)->update_light = false;
    
    int neighbors[4][2] = {{-1, 0}, {-1, 0}, {-1, 0}, {-1, 0}};
    
    if(x != 0) {
        neighbors[0][0] = x - 1;
        neighbors[0][1] = y;
    }
    if(x != blocks->getWidth() - 1) {
        neighbors[1][0] = x + 1;
        neighbors[1][1] = y;
    }
    if(y != 0) {
        neighbors[2][0] = x;
        neighbors[2][1] = y - 1;
    }
    if(y != blocks->getHeight() - 1) {
        neighbors[3][0] = x;
        neighbors[3][1] = y + 1;
    }
    
    if(!getLight(x, y)->light_source) {
        unsigned char level_to_be = 0;
        for(auto & neighbor : neighbors)
            if(neighbor[0] != -1) {
                unsigned char light_step = blocks->getBlockInfo(neighbor[0], neighbor[1]).transparent ? 3 : 15;
                unsigned char light = light_step > getLightLevel(neighbor[0], neighbor[1]) ? 0 : getLightLevel(neighbor[0], neighbor[1]) - light_step;
                if(light > level_to_be)
                    level_to_be = light;
            }
        setLightLevel(x, y, level_to_be);
    }
}

void Lights::setLightSource(unsigned short x, unsigned short y, unsigned char level) {
    getLight(x, y)->light_source = true;
    getLight(x, y)->light_level = level;
}

unsigned char Lights::getLightLevel(unsigned short x, unsigned short y) {
    return getLight(x, y)->light_level;
}

void Lights::scheduleLightUpdate(unsigned short x, unsigned short y) {
    getLight(x, y)->update_light = true;
}

bool Lights::hadScheduledLightUpdate(unsigned short x, unsigned short y) {
    return getLight(x, y)->update_light;
}
