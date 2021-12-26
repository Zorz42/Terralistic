#pragma once
#include "clientBlocks.hpp"

#define LIQUID_CHUNK_SIZE 16

class ClientLiquids : public Liquids, public ClientModule, EventListener<ClientPacketEvent>, EventListener<WelcomePacketEvent>, EventListener<LiquidChangeEvent> {
    class LiquidChunk {
        gfx::RectArray liquid_rects;
        bool is_created = false;
    public:
        bool isCreated() { return is_created; }
        void create(ClientLiquids* liquids, int x, int y);
        void update(ClientLiquids* liquids, int x, int y);
        void render(ClientLiquids* liquids, int x, int y);
    };
    
    LiquidChunk* liquid_chunks = nullptr;
    
    gfx::TextureAtlas liquids_atlas;
    
    ClientBlocks* blocks;
    ResourcePack* resource_pack;
    ClientNetworking* networking;
    Camera* camera;
    
    LiquidChunk* getLiquidChunk(int x, int y);
    
    void onEvent(ClientPacketEvent& event) override;
    void onEvent(WelcomePacketEvent& event) override;
    void onEvent(LiquidChangeEvent& event) override;
    
    void init() override;
    void postInit() override;
    void loadTextures() override;
    void render() override;
    void stop() override;
public:
    ClientLiquids(ClientBlocks* blocks, ResourcePack* resource_pack, ClientNetworking* networking, Camera* camera) : Liquids(blocks), resource_pack(resource_pack), networking(networking), blocks(blocks), camera(camera) {}
    
    const gfx::Texture& getLiquidsAtlasTexture();
    gfx::RectShape getLiquidRectInAtlas(LiquidType* type);
};
