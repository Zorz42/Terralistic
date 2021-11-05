#pragma once

#include "blocks.hpp"

#define MAX_LIGHT 100

class Lights : EventListener<BlockChangeEvent> {
    class Light {
    public:
        bool light_source = false, update_light = true;
        unsigned char light_level = 0;
    };
    
    Light* lights;
    
    Light* getLight(int x, int y);
    void setLightLevel(int x, int y, unsigned char level);
    
    void onEvent(BlockChangeEvent& event) override;
    
    Blocks* blocks;
public:
    Lights(Blocks* blocks) : blocks(blocks) {}
    void create();
    
    void init();
    void stop();
    
    int getWidth() const;
    int getHeight() const;
    
    void updateLight(int x, int y);
    void setLightSource(int x, int y, unsigned char level);
    unsigned char getLightLevel(int x, int y);
    void scheduleLightUpdate(int x, int y);
    bool hasScheduledLightUpdate(int x, int y);
    void scheduleLightUpdateForNeighbors(int x, int y);
    
    ~Lights();
};

class LightOutOfBoundsException : public Exception {
public:
    LightOutOfBoundsException(int x, int y) : Exception("Light is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")") {}
};
