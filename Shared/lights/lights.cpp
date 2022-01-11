#include "lights.hpp"

bool LightColor::operator==(LightColor color) {
    return r == color.r && g == color.g && b == color.b;
}

bool LightColor::operator!=(LightColor color) {
    return !(*this == color);
}

void Lights::init() {
    blocks->block_change_event.addListener(this);
}

void Lights::stop() {
    blocks->block_change_event.removeListener(this);
}

void Lights::create() {
    lights = new Light[blocks->getWidth() * blocks->getHeight()];
}

void Lights::updateAllLightEmitters() {
    for(int x = 0; x < getWidth(); x++)
        for(int y = 0; y < getHeight(); y++)
            updateLightEmitter(x, y);
}

Lights::Light* Lights::getLight(int x, int y) {
    if(x < 0 || x >= blocks->getWidth() || y < 0 || y >= blocks->getHeight())
        throw Exception("Light is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
    return &lights[y * blocks->getWidth() + x];
}

void Lights::setLightColor(int x, int y, LightColor color) {
    if(color.r == 0 && color.g == 0 && color.b == 0)
        getLight(x, y)->light_source = false;
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
    
    if(getLight(x, y)->light_source)
        return;
    
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
    for(int i = 0; i < 4; i++)
        if(neighbours[i][0] != -1) {
            int light_step = blocks->getBlockType(neighbours[i][0], neighbours[i][1])->transparent ? 3 : 15;
            
            int r = light_step > getLightColor(neighbours[i][0], neighbours[i][1]).r ? 0 : getLightColor(neighbours[i][0], neighbours[i][1]).r - light_step;
            if(r > color_to_be.r)
                color_to_be.r = r;
            
            int g = light_step > getLightColor(neighbours[i][0], neighbours[i][1]).g ? 0 : getLightColor(neighbours[i][0], neighbours[i][1]).g - light_step;
            if(g > color_to_be.g)
                color_to_be.g = g;
            
            int b = light_step > getLightColor(neighbours[i][0], neighbours[i][1]).b ? 0 : getLightColor(neighbours[i][0], neighbours[i][1]).b - light_step;
            if(b > color_to_be.b)
                color_to_be.b = b;
        }
    
    if(color_to_be != getLightColor(x, y)) {
        setLightColor(x, y, color_to_be);
        scheduleLightUpdate(x, y);
    }
}

void Lights::setLightSource(int x, int y, LightColor color) {
    getLight(x, y)->light_source = true;
    setLightColor(x, y, color);
    scheduleLightUpdateForNeighbors(x, y);
}

int Lights::getLightLevel(int x, int y) {
    return ((int)getLight(x, y)->color.r + (int)getLight(x, y)->color.g + (int)getLight(x, y)->color.b) / 3;
}

LightColor Lights::getLightColor(int x, int y) {
    return getLight(x, y)->color;
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
    updateLightEmitter(event.x, event.y);
}

void Lights::scheduleLightUpdateForNeighbors(int x, int y) {
    if(x < 0 || x >= blocks->getWidth() || y < 0 || y >= blocks->getHeight())
        throw Exception("Light is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
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
    setLightSource(x, y, LightColor(((rand() % 100) * blocks->getBlockType(x, y)->light_emission) % 100, ((rand() % 100) * blocks->getBlockType(x, y)->light_emission) % 100, ((rand() % 100) * blocks->getBlockType(x, y)->light_emission) % 100));
}

Lights::~Lights() {
    delete[] lights;
}
