#pragma once
#include <thread>
#include "clientBlocks.hpp"
#include "lights.hpp"

class NaturalLight : public ClientModule, EventListener<BlockChangeEvent>, EventListener<WelcomePacketEvent> {
    ClientNetworking* networking;
    ClientBlocks* blocks;
    Lights* lights;
    gfx::Timer timer, server_timer;
    float server_time_on_join = 0;
    
    bool running = true;
    std::thread natural_light_thread;
    
    void init() override;
    void postInit() override;
    void naturalLightUpdateLoop();
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
