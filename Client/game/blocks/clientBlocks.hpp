#ifndef clientBlocks_hpp
#define clientBlocks_hpp

#include <string>
#include "graphics.hpp"
#include "clientNetworking.hpp"
#include "properties.hpp"
#include "resourcePack.hpp"

#define BLOCK_WIDTH 8
#define MAX_LIGHT 100

class ClientBlocks;

class ClientMapBlock {
public:
    explicit ClientMapBlock(BlockType block_id=BlockType::AIR, LiquidType liquid_id=LiquidType::EMPTY) : block_id(block_id), liquid_id(liquid_id) {}

    BlockType block_id:8;
    LiquidType liquid_id:8;
    unsigned char light_level = 0, break_stage = 0, orientation = 0, liquid_level = 0;
};

class ClientBlock {
    ClientMapBlock* block_data;
    unsigned short x, y;
    ClientBlocks* parent_map;

public:
    ClientBlock(unsigned short x, unsigned short y, ClientMapBlock* block_data, ClientBlocks* parent_map) : x(x), y(y), block_data(block_data), parent_map(parent_map) {}
    
    void updateOrientation();
    unsigned char getOrientation() { return block_data->orientation; }
    
    void setType(BlockType block_id, LiquidType liquid_id);
    const BlockInfo& getBlockInfo() { return ::getBlockInfo(getBlockType()); }
    BlockType getBlockType() { return block_data->block_id; }
    
    const LiquidInfo& getLiquidInfo() { return ::getLiquidInfo(getLiquidType()); }
    LiquidType getLiquidType() { return block_data->liquid_id; }
    void setLiquidLevel(unsigned char level) { block_data->liquid_level = level; }
    unsigned char getLiquidLevel() { return block_data->liquid_level; }
    
    unsigned char getLightLevel() { return block_data->light_level; }
    void setLightLevel(unsigned char level);
    unsigned char getBreakStage() { return block_data->break_stage; }
    void setBreakStage(unsigned char stage);
};

class ClientBlocks : public gfx::GraphicalModule, EventListener<ClientPacketEvent> {
    unsigned short width{}, height{};
    ClientMapBlock *blocks = nullptr;

    networkingManager* networking_manager;
    
    void onEvent(ClientPacketEvent& event) override;
    
    ResourcePack* resource_pack;
    sf::VertexArray vertex_array;
    
    short getViewBeginX();
    short getViewEndX();
    short getViewBeginY();
    short getViewEndY();
    
public:
    ClientBlocks(networkingManager* manager, ResourcePack* resource_pack) : networking_manager(manager), resource_pack(resource_pack) {}
    int view_x{}, view_y{};

    ResourcePack* getResourcePack() { return resource_pack; }
    
    ClientBlock getBlock(unsigned short x, unsigned short y);

    void renderBackBlocks();
    void renderFrontBlocks();
    
    unsigned short getWorldWidth() const { return width; }
    unsigned short getWorldHeight() const { return height; }

    void createWorld(unsigned short map_width, unsigned short map_height);
    
    void updateVertexArray();

    ~ClientBlocks() override;
};

#endif
