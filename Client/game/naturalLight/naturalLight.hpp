#pragma once
#include <thread>
#include "clientBlocks.hpp"
#include "lights.hpp"
#include "debugMenu.hpp"

#define SECONDS_PER_DAY (10 * 60)

class NaturalLight : public ClientModule, EventListener<BlockChangeEvent>, EventListener<ClientPacketEvent> {
    ClientNetworking* networking;
    ClientBlocks* blocks;
    Lights* lights;
    gfx::ModTimer server_timer;
    DebugMenu* debug_menu;
    DebugLine day_time_line;
    
    bool running = true;
    std::thread natural_light_thread;
    
    void init() override;
    void postInit() override;
    void naturalLightUpdateLoop();
    void stop() override;
    
    long long getTime() const;
    int light_should_be = 0;
    int* lights_arr = nullptr;
    
    void onEvent(BlockChangeEvent& event) override;
    void onEvent(ClientPacketEvent& event) override;
    
    void setNaturalLight(int x, int power);
    void removeNaturalLight(int x);
public:
    NaturalLight(DebugMenu* debug_menu, ClientNetworking* networking, ClientBlocks* blocks, Lights* lights) : ClientModule("NaturalLight"), debug_menu(debug_menu), networking(networking), blocks(blocks), lights(lights), server_timer(SECONDS_PER_DAY * 1000) {}
    
    void updateLight(int x);
};
