#include "dayCycle.hpp"
#include "graphics.hpp"

#define DAY_STAGE_LENGTH 30000

static std::vector<unsigned char> day_stages = {
    (unsigned char)(1 * MAX_LIGHT),
    (unsigned char)(1 * MAX_LIGHT),
    (unsigned char)(0.95 * MAX_LIGHT),
    (unsigned char)(0.9 * MAX_LIGHT),
    (unsigned char)(0.7 * MAX_LIGHT),
    (unsigned char)(0.5 * MAX_LIGHT),
    (unsigned char)(0.15 * MAX_LIGHT),
    (unsigned char)(0.1 * MAX_LIGHT),
    (unsigned char)(0.05 * MAX_LIGHT),
    (unsigned char)(0.05 * MAX_LIGHT),
    (unsigned char)(0.05 * MAX_LIGHT),
    (unsigned char)(0.1 * MAX_LIGHT),
    (unsigned char)(0.15 * MAX_LIGHT),
    (unsigned char)(0.5 * MAX_LIGHT),
    (unsigned char)(0.7 * MAX_LIGHT),
    (unsigned char)(0.9 * MAX_LIGHT),
    (unsigned char)(0.95 * MAX_LIGHT),
    (unsigned char)(1 * MAX_LIGHT),
};

void DayCycle::init() {
    blocks->block_change_event.addListener(this);
    lights = new unsigned char[blocks->getWidth()];
    lights_should_be = new unsigned char[blocks->getWidth()];
    
    for(int x = 0; x < blocks->getWidth(); x++) {
        lights[x] = day_stages[0];
        for(unsigned short y = 0; y < blocks->getHeight() && blocks->getBlock(x, y).getBlockInfo().transparent; y++) {
            blocks->setLightLevelDirectly(x, y, day_stages[0]);
            blocks->setLightSourceDirectly(x, y, true);
        }
    }
    
    started = gfx::getTicks();
}

void DayCycle::onEvent(ServerBlockChangeEvent &event) {
    setNaturalLight(event.block.getX(), 0);
}

void DayCycle::update() {
    int curr_stage = ((gfx::getTicks() - started) / DAY_STAGE_LENGTH) % day_stages.size();
    int next_stage = (curr_stage + 1) % day_stages.size();
    int x = 0;
    for(; x < (gfx::getTicks() % DAY_STAGE_LENGTH) * blocks->getWidth() / DAY_STAGE_LENGTH; x++)
        lights_should_be[x] = day_stages[next_stage];
    
    for(; x < blocks->getWidth(); x++)
        lights_should_be[x] = day_stages[curr_stage];
    
    //for(x = 0; x < blocks->getWidth(); x++)
        //if(lights[x] != lights_should_be[x])
            //setNaturalLight(x, lights_should_be[x]);
}

void DayCycle::setNaturalLight(unsigned short x, unsigned char power) {
    lights[x] = power;
    for(unsigned short y = 0; y < blocks->getHeight() && blocks->getBlock(x, y).getBlockInfo().transparent; y++)
        blocks->getBlock(x, y).setLightSource(power);
}
