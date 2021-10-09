#pragma once

#include "graphics.hpp"
#include "clientNetworking.hpp"
#include "resourcePack.hpp"
#include "blocks.hpp"

class ClientBlocks : public Blocks, EventListener<ClientPacketEvent>, EventListener<BlockChangeEvent> {
    struct RenderBlock {
        unsigned char variation = rand(), state = 16;
    };
    
    void onEvent(ClientPacketEvent& event) override;
    void onEvent(BlockChangeEvent& event) override;
    
    RenderBlock* render_blocks = nullptr;
    RenderBlock* getRenderBlock(int x, int y);
    
    std::vector<void (*)(ClientBlocks*, int, int)> stateFunctions[(int)BlockType::NUM_BLOCKS];
    
    ResourcePack* resource_pack;
    NetworkingManager* networking;
public:
    ClientBlocks(ResourcePack* resource_pack, NetworkingManager* networking);
    
    void init();
    void render();
    void stop();
    
    void create();
    
    int view_x, view_y;
    
    short getViewBeginX() const;
    short getViewEndX() const;
    short getViewBeginY() const;
    short getViewEndY() const;
    
    void updateState(int x, int y);
    void setState(int x, int y, unsigned char state);
    unsigned char getState(int x, int y);
    
    void onWelcomePacket(sf::Packet& packet, WelcomePacketType type);
    
    ~ClientBlocks();
};
