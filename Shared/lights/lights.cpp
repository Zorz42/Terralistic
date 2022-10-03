#include "lights.hpp"

bool LightColor::operator==(LightColor color) const {
    return r == color.r && g == color.g && b == color.b;
}

bool LightColor::operator!=(LightColor color) const {
    return !(*this == color);
}

void Lights::init() {
    blocks->block_change_event.addListener(this);
}

void Lights::stop() {
    blocks->block_change_event.removeListener(this);
}

void Lights::create() {
    lights.resize(blocks->getWidth() * blocks->getHeight());
}

Lights::Light* Lights::getLight(int x, int y) {
    if(x < 0 || x >= blocks->getWidth() || y < 0 || y >= blocks->getHeight() || lights.empty())
        throw Exception("Light is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
    return &lights[y * blocks->getWidth() + x];
}

bool Lights::isLightSource(int x, int y) {
    return getLight(x, y)->light_source;
}

void Lights::setLightColor(int x, int y, LightColor color) {
    if(getLight(x, y)->color != color) {
        getLight(x, y)->color = color;
        LightColorChangeEvent event(x, y);
        light_color_change_event.call(event);
        
        if(x < getWidth() - 1)
            scheduleLightUpdate(x + 1, y);
        if(x > 0)
            scheduleLightUpdate(x - 1, y);
        if(y < getHeight() - 1)
            scheduleLightUpdate(x, y + 1);
        if(y > 0)
            scheduleLightUpdate(x, y - 1);
    }
}

void Lights::updateLight(int x, int y) {
    getLight(x, y)->update_light = false;
    updateLightEmitter(x, y);
    
    int neighbours[4][2] = {{-1, 0}, {-1, 0}, {-1, 0}, {-1, 0}};
    
    if(x != 0) {
        neighbours[0][0] = x - 1;
        neighbours[0][1] = y;
    }
    if(x != blocks->getWidth() - 1) {
        neighbours[1][0] = x + 1;
        neighbours[1][1] = y;
    }
    if(y != 0) {
        neighbours[2][0] = x;
        neighbours[2][1] = y - 1;
    }
    if(y != blocks->getHeight() - 1) {
        neighbours[3][0] = x;
        neighbours[3][1] = y + 1;
    }
    
    LightColor color_to_be(0, 0, 0);
    for(auto & neighbour : neighbours)
        if(neighbour[0] != -1) {
            int light_step = blocks->getBlockType(neighbour[0], neighbour[1])->transparent ? 3 : 15;
            
            int r = light_step > getLightColor(neighbour[0], neighbour[1]).r ? 0 : getLightColor(neighbour[0], neighbour[1]).r - light_step;
            if(r > color_to_be.r)
                color_to_be.r = r;
            
            int g = light_step > getLightColor(neighbour[0], neighbour[1]).g ? 0 : getLightColor(neighbour[0], neighbour[1]).g - light_step;
            if(g > color_to_be.g)
                color_to_be.g = g;
            
            int b = light_step > getLightColor(neighbour[0], neighbour[1]).b ? 0 : getLightColor(neighbour[0], neighbour[1]).b - light_step;
            if(b > color_to_be.b)
                color_to_be.b = b;
        }
    
    if(isLightSource(x, y)) {
        color_to_be.r = std::max(color_to_be.r, getLight(x, y)->source_color.r);
        color_to_be.g = std::max(color_to_be.g, getLight(x, y)->source_color.g);
        color_to_be.b = std::max(color_to_be.b, getLight(x, y)->source_color.b);
    }
    
    if(color_to_be != getLightColor(x, y)) {
        setLightColor(x, y, color_to_be);
        scheduleLightUpdate(x, y);
    }
}

void Lights::setLightSource(int x, int y, LightColor color) {
    getLight(x, y)->light_source = color != LightColor(0, 0, 0);
    if(getLight(x, y)->source_color != color) {
        getLight(x, y)->source_color = color;
        scheduleLightUpdateForNeighbors(x, y);
    }
}

LightColor Lights::getLightColor(int x, int y) {
    return getLight(x, y)->color;
}

LightColor Lights::getLightSourceColor(int x, int y) {
    return getLight(x, y)->source_color;
}

void Lights::scheduleLightUpdate(int x, int y) {
    getLight(x, y)->update_light = true;
    LightUpdateScheduleEvent event(x, y);
    light_update_schedule_event.call(event);
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
    scheduleLightUpdate(x, y);
    if(x != 0)
        scheduleLightUpdate(x - 1, y);
    if(x != blocks->getWidth() - 1)
        scheduleLightUpdate(x + 1, y);
    if(y != 0)
        scheduleLightUpdate(x, y - 1);
    if(y != blocks->getHeight() - 1)
        scheduleLightUpdate(x, y + 1);
}

void Lights::updateLightEmitter(int x, int y) {
    BlockType* block_type = blocks->getBlockType(x, y);
    if(block_type != &blocks->air)
        setLightSource(x, y, LightColor(block_type->light_emission_r, block_type->light_emission_g, block_type->light_emission_b));
}
