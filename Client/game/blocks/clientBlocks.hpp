#ifndef clientBlocks_hpp
#define clientBlocks_hpp

#include <string>
#include "graphics.hpp"
#include "clientNetworking.hpp"
#include "properties.hpp"
#include "resourcePack.hpp"
#include "liquids.hpp"
#include "lights.hpp"

#define BLOCK_WIDTH 8
#define MAX_LIGHT 100

class ClientBlocks : EventListener<ClientPacketEvent> {
    struct ClientBlock {
        unsigned char variation = rand(), state = 16;
    };
    
    void onEvent(ClientPacketEvent& event) override;
    
    ClientBlock* client_blocks;
    ClientBlock* getClientBlock(unsigned short x, unsigned short y);

    ResourcePack* resource_pack;
    NetworkingManager* manager;
    Blocks* blocks;
    Liquids* liquids;
    Lights* lights;
public:
    explicit ClientBlocks(ResourcePack* resource_pack, NetworkingManager* manager, Blocks* blocks, Liquids* liquids, Lights* lights);
    
    std::vector<void (*)(Blocks*, ClientBlocks*, unsigned short, unsigned short)> stateFunctions[(int)BlockType::NUM_BLOCKS];
    int view_x, view_y;

    void init();
    
    void create();
    
    ResourcePack* getResourcePack() { return resource_pack; }

    void renderBackBlocks();
    void renderFrontBlocks();
    
    short getViewBeginX() const;
    short getViewEndX() const;
    short getViewBeginY() const;
    short getViewEndY() const;
    
    void updateState(unsigned short x, unsigned short y);
    
    void setState(unsigned short x, unsigned short y, unsigned char state);
    unsigned char getState(unsigned short x, unsigned short y);

    ~ClientBlocks();
};

#endif
