#include "lights.hpp"

void Lights::init() {
    blocks->block_change_event.addListener(this);
}

void Lights::stop() {
    blocks->block_change_event.removeListener(this);
}

void Lights::create() {
    lights = new Light[blocks->getWidth() * blocks->getHeight()];
}

Lights::Light* Lights::getLight(int x, int y) {
    if(x >= blocks->getWidth() || y >= blocks->getHeight())
        throw LightOutOfBoundsException();
    return &lights[y * blocks->getWidth() + x];
}

void Lights::setLightLevel(int x, int y, unsigned char level) {
    if(level == 0)
        getLight(x, y)->light_source = false;
    if(getLight(x, y)->light_level != level) {
        getLight(x, y)->light_level = level;
        if(x < getWidth())
            scheduleLightUpdate(x + 1, y);
        if(x > 0)
            scheduleLightUpdate(x - 1, y);
        if(y < getHeight())
            scheduleLightUpdate(x, y + 1);
        if(y > 0)
            scheduleLightUpdate(x, y - 1);
    }
}

void Lights::updateLight(int x, int y) {
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
                unsigned char light_step = blocks->getBlockType(neighbor[0], neighbor[1])->transparent ? 3 : 15;
                unsigned char light = light_step > getLightLevel(neighbor[0], neighbor[1]) ? 0 : getLightLevel(neighbor[0], neighbor[1]) - light_step;
                if(light > level_to_be)
                    level_to_be = light;
            }
        if(level_to_be != getLightLevel(x, y)) {
            setLightLevel(x, y, level_to_be);
            scheduleLightUpdate(x, y);
        }
    }
}

void Lights::setLightSource(int x, int y, unsigned char level) {
    getLight(x, y)->light_source = level > 0;
    getLight(x, y)->light_level = level;
    scheduleLightUpdateForNeighbors(x, y);
}

unsigned char Lights::getLightLevel(int x, int y) {
    return getLight(x, y)->light_level;
}

void Lights::scheduleLightUpdate(int x, int y) {
    getLight(x, y)->update_light = true;
}

bool Lights::hasScheduledLightUpdate(int x, int y) {
    return getLight(x, y)->update_light;
}

int Lights::getWidth() const {
    return blocks->getWidth();
}

int Lights::getHeight() const {
    return blocks->getHeight();
}

void Lights::onEvent(BlockChangeEvent& event) {
    scheduleLightUpdate(event.x, event.y);
}

void Lights::scheduleLightUpdateForNeighbors(int x, int y) {
    if(x != 0)
        scheduleLightUpdate(x - 1, y);
    if(x != blocks->getWidth() - 1)
        scheduleLightUpdate(x + 1, y);
    if(y != 0)
        scheduleLightUpdate(x, y - 1);
    if(y != blocks->getHeight() - 1)
        scheduleLightUpdate(x, y + 1);
}

Lights::~Lights() {
    delete[] lights;
}
