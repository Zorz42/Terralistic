#pragma once
#include "clientBlocks.hpp"
#include "lights.hpp"
#include "settings.hpp"

#define LIGHT_CHUNK_SIZE 16

class ClientLights : public Lights, public ClientModule, EventListener<LightColorChangeEvent> {
    class LightChunk {
        gfx::RectArray light_rects;
        bool is_created = false;
    public:
        bool isCreated() { return is_created; }
        void create(ClientLights* lights, int x, int y);
        void update(ClientLights* lights, int x, int y);
        void render(int x, int y);
    };
    
    Settings* settings;
    ClientBlocks* blocks;
    ResourcePack* resource_pack;
    Camera* camera;
    
    void onEvent(LightColorChangeEvent& event) override;
    
    LightChunk* light_chunks = nullptr;
    
    void init() override;
    void postInit() override;
    void render() override;
    void update(float frame_length) override;
    void stop() override;
    
    BooleanSetting light_enable_setting;
    
    LightChunk* getLightChunk(int x, int y);
    
    bool* light_updates = nullptr;
    bool& getLightUpdate(int x, int y);
    
    LightColor getAverageColor(int x, int y);
public:
    ClientLights(Settings* settings, ClientBlocks* blocks, ResourcePack* resource_pack, Camera* camera) : Lights(blocks), settings(settings), blocks(blocks), resource_pack(resource_pack), camera(camera), light_enable_setting("Light", true) {}
};
