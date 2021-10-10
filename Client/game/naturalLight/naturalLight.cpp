#include <cmath>
#include "naturalLight.hpp"
#include "graphics.hpp"

#define SECONDS_PER_DAY (60 * 10)
//#define SECONDS_PER_DAY 5

float dayFunction(float a) {
    a -= (int)a;
    
    if(a < 0.25)
        return 1;
    if(a < 0.5)
        return -a * 4 + 2;
    if(a < 0.75)
        return 0;
    return a * 4 - 3;
}

void NaturalLight::init() {
    blocks->block_change_event.addListener(this);
    started = gfx::getTicks();
}

void NaturalLight::stop() {
    blocks->block_change_event.removeListener(this);
}

void NaturalLight::onEvent(BlockChangeEvent &event) {
    setNaturalLight(event.x, 0);
}

void NaturalLight::update(float frame_length) {
    light_should_be = dayFunction((float)getTime() / 1000 / SECONDS_PER_DAY) * MAX_LIGHT;
    
    if(light_should_be != prev_light_should_be) {
        for(int x = blocks->getViewBeginX(); x < blocks->getViewEndX(); x++)
            updateLight(x);
        prev_light_should_be = light_should_be;
    }
}

void NaturalLight::setNaturalLight(unsigned short x, unsigned char power) {
    for(unsigned short y = 0; y < blocks->getHeight() && blocks->getBlockInfo(x, y).transparent; y++)
        lights->setLightSource(x, y, power);
}

unsigned int NaturalLight::getTime() {
    return gfx::getTicks() - started;
}

void NaturalLight::updateLight(int x) {
    setNaturalLight(x, light_should_be);
}
