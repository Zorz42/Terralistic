#pragma once

#include "graphics.hpp"
#include "clientNetworking.hpp"
#include "resourcePack.hpp"
#include "blocks.hpp"
#include "lights.hpp"

class ClientBlocks : public Blocks, public ClientModule, EventListener<ClientPacketEvent>, EventListener<BlockChangeEvent>, EventListener<WelcomePacketEvent> {
    struct RenderBlock {
        unsigned char variation = rand(), state = 16;
    };
    
    void onEvent(ClientPacketEvent& event) override;
    void onEvent(BlockChangeEvent& event) override;
    void onEvent(WelcomePacketEvent& event) override;
    
    RenderBlock* render_blocks = nullptr;
    RenderBlock* getRenderBlock(int x, int y);
    
    std::vector<void (*)(ClientBlocks*, int, int)> stateFunctions[(int)BlockType::NUM_BLOCKS];
    
    void init() override;
    void postInit() override;
    void render() override;
    void update(float frame_length) override;
    void stop() override;
    
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
    
    ~ClientBlocks();
};
