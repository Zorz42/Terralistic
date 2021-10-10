#include <cmath>
#include "naturalLight.hpp"
#include "graphics.hpp"

#define SECONDS_PER_DAY (60 * 10)

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
    light_should_be = (sin((float)getTime() * 2 * M_PI / 1000 / SECONDS_PER_DAY + M_PI_2) + 1) / 2 * MAX_LIGHT;
    
    for(int x = blocks->getViewBeginX(); x < blocks->getViewEndX(); x++)
        updateLight(x);
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
