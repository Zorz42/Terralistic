#pragma once

#include "graphics.hpp"
#include "clientNetworking.hpp"
#include "resourcePack.hpp"
#include "clientBlocks.hpp"
#include "lights.hpp"

class ClientLights : public Lights {
    ClientBlocks* blocks;
    ResourcePack* resource_pack;
public:
    ClientLights(ClientBlocks* blocks, ResourcePack* resource_pack) : Lights(blocks), blocks(blocks), resource_pack(resource_pack) {}
    
    void render();
    
    void updateLights();
};
