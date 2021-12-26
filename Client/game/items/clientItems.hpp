#pragma once
#include "clientBlocks.hpp"

#define ITEM_WIDTH 8

class ClientItems : public Items, public ClientModule, EventListener<ClientPacketEvent>, EventListener<ItemCreationEvent>, EventListener<EntityDeletionEvent> {
    ResourcePack* resource_pack;
    
    int item_count = 0;
    
    gfx::RectArray item_rects;
    gfx::TextureAtlas items_atlas;
    
    void onEvent(ClientPacketEvent& event) override;
    void onEvent(ItemCreationEvent& event) override;
    void onEvent(EntityDeletionEvent& event) override;
    
    ClientBlocks* blocks;
    Entities* entities;
    ClientNetworking* networking;
    Camera* camera;
    
    void init() override;
    void loadTextures() override;
    void render() override;
    void stop() override;
public:
    ClientItems(ResourcePack* resource_pack, ClientBlocks* blocks, Entities* entities, ClientNetworking* networking, Camera* camera) :  Items(entities, blocks), resource_pack(resource_pack), blocks(blocks), entities(entities), networking(networking), camera(camera) {}
    
    const gfx::Texture& getItemsAtlasTexture();
    gfx::RectShape getItemRectInAtlas(ItemType* type);
};

