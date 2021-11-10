#pragma once

#include "blocks.hpp"

#define MAX_LIGHT 100

class Lights : EventListener<BlockChangeEvent> {
    class Light {
    public:
        Light() : light_level(0) {}
        bool light_source = false, update_light = true;
        int light_level:8;
    };
    
    Light* lights;
    
    Light* getLight(int x, int y);
    void setLightLevel(int x, int y, int level);
    
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
    void setLightSource(int x, int y, int level);
    int getLightLevel(int x, int y);
    void scheduleLightUpdate(int x, int y);
    bool hasScheduledLightUpdate(int x, int y);
    void scheduleLightUpdateForNeighbors(int x, int y);
    
    ~Lights();
};
