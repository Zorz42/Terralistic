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
    explicit ClientMapBlock(BlockType block_id=BlockType::AIR, LiquidType liquid_id=LiquidType::EMPTY, unsigned char liquid_level=0, unsigned char light_level=0) : block_id(block_id), liquid_id(liquid_id), liquid_level(liquid_level), light_level(light_level) {}

    BlockType block_id:8;
    LiquidType liquid_id:8;
    unsigned char light_level, break_stage = 0, liquid_level, variation = rand(), state = 16;
};

class ClientBlock {
    ClientMapBlock* block_data;
    unsigned short x, y;
    ClientBlocks* blocks;

public:
    ClientBlock(unsigned short x, unsigned short y, ClientMapBlock* block_data, ClientBlocks* blocks) : x(x), y(y), block_data(block_data), blocks(blocks) {}
    
    void updateState();

    unsigned char getState() {return block_data->state;}
    void setState(unsigned char state);
    
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
    unsigned  char getVariation() {return block_data->variation; }
};

class ClientBlocks : public gfx::SceneModule, EventListener<ClientPacketEvent> {
    unsigned short width{}, height{};
    ClientMapBlock *blocks = nullptr;
    
    void onEvent(ClientPacketEvent& event) override;
    
    ResourcePack* resource_pack;

    bool updateOrientationSide(unsigned short x, unsigned short y, char side_x, char side_y);
    void updateOrientationDown(unsigned short x, unsigned short y);
    void updateOrientationUp(unsigned short x, unsigned short y);
    void updateOrientationLeft(unsigned short x, unsigned short y);
    void updateOrientationRight(unsigned short x, unsigned short y);

public:
    explicit ClientBlocks(ResourcePack* resource_pack);
    void create(unsigned short map_width, unsigned short map_height, const std::vector<char>& map_data);
    std::vector<void (ClientBlocks::*)(unsigned short, unsigned short)> stateFunctions[(int)BlockType::NUM_BLOCKS];
    int view_x{}, view_y{};

    ResourcePack* getResourcePack() { return resource_pack; }
    
    ClientBlock getBlock(unsigned short x, unsigned short y);

    void renderBackBlocks();
    void renderFrontBlocks();
    
    unsigned short getWidth() const { return width; }
    unsigned short getHeight() const { return height; }
    
    short getViewBeginX() const;
    short getViewEndX() const;
    short getViewBeginY() const;
    short getViewEndY() const;

    ~ClientBlocks() override;
};

#endif
