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

void NaturalLight::postInit() {
    lights_arr = new int[blocks->getWidth()];
    for(int x = 0; x < blocks->getWidth(); x++)
        lights_arr[x] = 0;
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

    for(int x = blocks->getViewBeginX(); x < blocks->getViewEndX(); x++)
        updateLight(x);

}

void NaturalLight::setNaturalLight(int x, int power) {
    if(lights_arr[x] != power) {
        lights_arr[x] = power;
        for(int y = 0; y < blocks->getHeight() && blocks->getBlockType(x, y)->transparent; y++)
            lights->setLightSource(x, y, power);
    }
}

void NaturalLight::removeNaturalLight(int x) {
    lights_arr[x] = 0;
    for(int y = 0; y < blocks->getHeight(); y++)
        lights->setLightSource(x, y, 0);
}

int NaturalLight::getTime() const {
    return server_time_on_join + gfx::getTicks() - started;
}

void NaturalLight::updateLight(int x) {
    setNaturalLight(x, light_should_be);
}
