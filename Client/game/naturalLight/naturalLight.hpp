#pragma once

#include "clientBlocks.hpp"
#include "events.hpp"
#include "lights.hpp"
#include "clientModule.hpp"

class NaturalLight : public ClientModule, EventListener<BlockChangeEvent>, EventListener<WelcomePacketEvent> {
    ClientNetworking* networking;
    ClientBlocks* blocks;
    Lights* lights;
    unsigned int started = 0, server_time_on_join = 0;
    
    void init() override;
    void update(float frame_length) override;
    void stop() override;
    
    unsigned int getTime();
    int light_should_be = 0, prev_light_should_be = 0;
    
    void onEvent(BlockChangeEvent& event) override;
    void onEvent(WelcomePacketEvent& event) override;
    
    void setNaturalLight(int x, unsigned char power);
    void removeNaturalLight(int x);
public:
    NaturalLight(ClientNetworking* networking, ClientBlocks* blocks, Lights* lights) : networking(networking), blocks(blocks), lights(lights) {}
    
    void updateLight(int x);
};
