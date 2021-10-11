#include <cmath>
#include "naturalLight.hpp"
#include "graphics.hpp"

#define SECONDS_PER_DAY (60 * 10)

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
    networking->welcome_packet_event.addListener(this);
}

void NaturalLight::stop() {
    blocks->block_change_event.removeListener(this);
    networking->welcome_packet_event.removeListener(this);
}

void NaturalLight::onEvent(BlockChangeEvent &event) {
    removeNaturalLight(event.x);
    updateLight(event.x);
}

void NaturalLight::onEvent(WelcomePacketEvent &event) {
    if(event.packet_type == WelcomePacketType::TIME) {
        event.packet >> server_time_on_join;
        started = gfx::getTicks();
    }
}

void NaturalLight::update(float frame_length) {
    light_should_be = dayFunction((float)getTime() / 1000 / SECONDS_PER_DAY) * MAX_LIGHT;
    
    if(light_should_be != prev_light_should_be) {
        for(int x = blocks->getViewBeginX(); x < blocks->getViewEndX(); x++)
            updateLight(x);
        prev_light_should_be = light_should_be;
    }
}

void NaturalLight::setNaturalLight(int x, unsigned char power) {
    for(int y = 0; y < blocks->getHeight() && blocks->getBlockInfo(x, y).transparent; y++)
        lights->setLightSource(x, y, power);
}

void NaturalLight::removeNaturalLight(int x) {
    for(int y = 0; y < blocks->getHeight(); y++)
        lights->setLightSource(x, y, 0);
}

unsigned int NaturalLight::getTime() {
    return server_time_on_join + gfx::getTicks() - started;
}

void NaturalLight::updateLight(int x) {
    setNaturalLight(x, light_should_be);
}
