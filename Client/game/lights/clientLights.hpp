#pragma once

#include "graphics.hpp"
#include "clientNetworking.hpp"
#include "resourcePack.hpp"
#include "clientBlocks.hpp"
#include "lights.hpp"

class ClientLights : public Lights, public ClientModule {
    ClientBlocks* blocks;
    ResourcePack* resource_pack;
    
    void init() override;
    void postInit() override;
    void render() override;
    void update(float frame_length) override;
    void stop() override;
public:
    ClientLights(ClientBlocks* blocks, ResourcePack* resource_pack) : Lights(blocks), blocks(blocks), resource_pack(resource_pack) {}
};
