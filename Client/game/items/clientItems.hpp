#pragma once

#include "graphics.hpp"
#include "resourcePack.hpp"
#include "events.hpp"
#include "clientNetworking.hpp"
#include "clientBlocks.hpp"
#include "clientEntities.hpp"
#include "items.hpp"

#define ITEM_WIDTH 8

class ClientItems : public Items, public ClientModule, EventListener<ClientPacketEvent>, EventListener<ItemCreationEvent>, EventListener<EntityDeletionEvent> {
    ResourcePack* resource_pack;
    
    int item_count = 0;
    
    gfx::RectArray item_rects;
    
    void onEvent(ClientPacketEvent& event) override;
    void onEvent(ItemCreationEvent& event) override;
    void onEvent(EntityDeletionEvent& event) override;
    
    ClientBlocks* blocks;
    Entities* entities;
    ClientNetworking* manager;
    
    void init() override;
    void render() override;
    void stop() override;
public:
    ClientItems(ResourcePack* resource_pack, ClientBlocks* blocks, Entities* entities, ClientNetworking* manager) : resource_pack(resource_pack), blocks(blocks), entities(entities), manager(manager), Items(entities, blocks) {}
};

