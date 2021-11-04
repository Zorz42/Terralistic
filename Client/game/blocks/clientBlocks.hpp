#pragma once

#include "graphics.hpp"
#include "clientNetworking.hpp"
#include "resourcePack.hpp"
#include "blocks.hpp"
#include "lights.hpp"

class ClientBlocks : public Blocks, public ClientModule, EventListener<ClientPacketEvent>, EventListener<BlockChangeEvent>, EventListener<WelcomePacketEvent> {
    class RenderBlock {
    public:
        unsigned char variation = rand(), state = 16;
    };
    
    void onEvent(ClientPacketEvent& event) override;
    void onEvent(BlockChangeEvent& event) override;
    void onEvent(WelcomePacketEvent& event) override;
    
    RenderBlock* render_blocks = nullptr;
    RenderBlock* getRenderBlock(int x, int y);
    
    void init() override;
    void postInit() override;
    void render() override;
    void update(float frame_length) override;
    void stop() override;
    
    bool updateOrientationSide(ClientBlocks* blocks, int x, int y, char side_x, char side_y);
    void updateOrientationDown(ClientBlocks* blocks, int x, int y);
    void updateOrientationUp(ClientBlocks* blocks, int x, int y);
    void updateOrientationLeft(ClientBlocks* blocks, int x, int y);
    void updateOrientationRight(ClientBlocks* blocks, int x, int y);
    
    gfx::RectArray block_rects;
    int most_blocks_on_screen = 0;
    
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
    void setState(int x, int y, unsigned char state);
    unsigned char getState(int x, int y);
    
    bool skip_rendering_in_dark = true;
    
    ~ClientBlocks();
};
