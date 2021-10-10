#pragma once

#include "clientBlocks.hpp"
#include "events.hpp"
#include "lights.hpp"
#include "clientModule.hpp"

class NaturalLight : public ClientModule, EventListener<BlockChangeEvent> {
    ClientBlocks* blocks;
    Lights* lights;
    unsigned int started;
    
    void init() override;
    void update(float frame_length) override;
    void stop() override;
    
    unsigned int getTime();
    int light_should_be = 0;
    
    void onEvent(BlockChangeEvent& event) override;
    
    void setNaturalLight(unsigned short x, unsigned char power);
public:
    NaturalLight(ClientBlocks* blocks, Lights* lights) : blocks(blocks), lights(lights) {}
    
    void updateLight(int x);
};
