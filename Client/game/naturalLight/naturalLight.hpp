#pragma once
#include "clientBlocks.hpp"
#include "lights.hpp"

class NaturalLight : public ClientModule, EventListener<BlockChangeEvent>, EventListener<WelcomePacketEvent> {
    ClientNetworking* networking;
    ClientBlocks* blocks;
    Lights* lights;
    gfx::Timer timer, server_timer;
    float server_time_on_join;
    
    void init() override;
    void postInit() override;
    void updateParallel(float frame_length) override;
    void stop() override;
    
    int getTime() const;
    int light_should_be = 0;
    int* lights_arr = nullptr;
    
    void onEvent(BlockChangeEvent& event) override;
    void onEvent(WelcomePacketEvent& event) override;
    
    void setNaturalLight(int x, int power);
    void removeNaturalLight(int x);
public:
    NaturalLight(ClientNetworking* networking, ClientBlocks* blocks, Lights* lights) : networking(networking), blocks(blocks), lights(lights) {}
    
    void updateLight(int x);
};
