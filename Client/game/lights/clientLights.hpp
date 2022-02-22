#pragma once
#include <thread>
#include "clientBlocks.hpp"
#include "lights.hpp"
#include "settings.hpp"

class ClientLights : public Lights, public ClientModule, EventListener<LightColorChangeEvent> {
    class LightChunk {
        gfx::RectArray light_rects;
        bool is_created = false;
        int lights_count;
    public:
        bool has_update = true;
        bool isCreated() { return is_created; }
        void create(ClientLights* lights, int x, int y);
        void update(ClientLights* lights, int x, int y);
        void render(int x, int y);
    };
    
    Settings* settings;
    ClientBlocks* blocks;
    ResourcePack* resource_pack;
    Camera* camera;
    
    bool running = true;
    std::thread light_update_thread;
    
    void onEvent(LightColorChangeEvent& event) override;
    
    LightChunk* light_chunks = nullptr;
    
    void init() override;
    void postInit() override;
    void render() override;
    void update(float frame_length) override;
    void updateParallel(float frame_length) override;
    void stop() override;
    
    void lightUpdateLoop();
    
    BooleanSetting light_enable_setting;
    
    LightChunk* getLightChunk(int x, int y);
    
    void scheduleLightUpdate(int x, int y);
public:
    ClientLights(Settings* settings, ClientBlocks* blocks, ResourcePack* resource_pack, Camera* camera) : Lights(blocks), settings(settings), blocks(blocks), resource_pack(resource_pack), camera(camera), light_enable_setting("Light", true) {}
};
