#pragma once

#include "graphics.hpp"
#include "clientNetworking.hpp"
#include "resourcePack.hpp"
#include "clientBlocks.hpp"
#include "lights.hpp"
#include "settings.hpp"

class ClientLights : public Lights, public ClientModule {
    Settings* settings;
    ClientBlocks* blocks;
    ResourcePack* resource_pack;
    
    gfx::RectArray light_rects;
    int most_blocks_on_screen = 0;
    
    void init() override;
    void postInit() override;
    void render() override;
    void update(float frame_length) override;
    void stop() override;
    
    BooleanSetting light_enable_setting;
public:
    ClientLights(Settings* settings, ClientBlocks* blocks, ResourcePack* resource_pack) : Lights(blocks), settings(settings), blocks(blocks), resource_pack(resource_pack), light_enable_setting("Light", true) {}
};
