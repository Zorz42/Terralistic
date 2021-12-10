#pragma once
#include "clientNetworking.hpp"
#include "resourcePack.hpp"
#include "lights.hpp"
#include "content.hpp"

#define BLOCK_CHUNK_SIZE 16

class ClientBlocks : public Blocks, public ClientModule, EventListener<ClientPacketEvent>, EventListener<BlockChangeEvent>, EventListener<WelcomePacketEvent> {
    class RenderBlock {
    public:
        RenderBlock() : variation(rand()), state(16) {}
        int variation:8, state:8;
    };
    
    class RenderBlockChunk {
        gfx::RectArray block_rects;
    public:
        RenderBlockChunk() : block_rects(BLOCK_CHUNK_SIZE * BLOCK_CHUNK_SIZE) {}
        void update(ClientBlocks* blocks, ResourcePack* resource_pack, int x, int y);
        void render(ResourcePack* resource_pack_, int x, int y);
    };
    
    void onEvent(ClientPacketEvent& event) override;
    void onEvent(BlockChangeEvent& event) override;
    void onEvent(WelcomePacketEvent& event) override;
    
    RenderBlock* render_blocks = nullptr;
    RenderBlockChunk* render_chunks = nullptr;
    RenderBlock* getRenderBlock(int x, int y);
    
    RenderBlockChunk* getRenderBlockChunk(int x, int y);
    
    void init() override;
    void postInit() override;
    void render() override;
    void update(float frame_length) override;
    void stop() override;
    
    bool updateOrientationSide(ClientBlocks* blocks, int x, int y, int side_x, int side_y);
    void updateOrientationDown(ClientBlocks* blocks, int x, int y);
    void updateOrientationUp(ClientBlocks* blocks, int x, int y);
    void updateOrientationLeft(ClientBlocks* blocks, int x, int y);
    void updateOrientationRight(ClientBlocks* blocks, int x, int y);
    
    ResourcePack* resource_pack;
    ClientNetworking* networking;
    Lights* lights;
    
public:
    ClientBlocks(ResourcePack* resource_pack, ClientNetworking* networking, Lights* lights);
    
    int view_x, view_y;
    
    int getViewBeginX() const;
    int getViewEndX() const;
    int getViewBeginY() const;
    int getViewEndY() const;
    
    void updateState(int x, int y);
    void setState(int x, int y, int state);
    int getState(int x, int y);
};
