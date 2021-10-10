#pragma once

#include "blocks.hpp"

#define MAX_LIGHT 100

class Lights : EventListener<BlockChangeEvent> {
    struct Light {
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
    
    unsigned short getWidth() const;
    unsigned short getHeight() const;
    
    void updateLight(int x, int y);
    void setLightSource(int x, int y, unsigned char level);
    unsigned char getLightLevel(int x, int y);
    void scheduleLightUpdate(int x, int y);
    bool hasScheduledLightUpdate(int x, int y);
    void scheduleLightUpdateForNeighbors(int x, int y);
    
    ~Lights();
};

class LightOutOfBoundsException : public std::exception {
public:
    const char* what() const throw() {
        return "Light is accessed out of the bounds!";
    }
};
