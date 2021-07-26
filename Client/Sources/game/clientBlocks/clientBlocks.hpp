#ifndef clientBlocks_hpp
#define clientBlocks_hpp

#include <string>

#include "graphics.hpp"
#include "clientNetworking.hpp"
#include "properties.hpp"
#include "resourcePack.hpp"

#define BLOCK_WIDTH 16
#define MAX_LIGHT 100

enum class ChunkState {unloaded, pending_load, loaded};

class ClientBlocks;

struct ClientMapBlock {
    explicit ClientMapBlock(BlockType block_id=BlockType::AIR, LiquidType liquid_id=LiquidType::EMPTY) : block_id(block_id), liquid_id(liquid_id) {}

    BlockType block_id;
    LiquidType liquid_id;
    unsigned char light_level = 0, break_stage = 0, orientation = 0, liquid_level = 0;
    bool update = true;

    const BlockInfo& getUniqueBlock() const;
    const LiquidInfo& getUniqueLiquid() const;
};

class ClientBlock {
    ClientMapBlock* block_data;
    unsigned short x, y;
    ClientBlocks* parent_map;

    void scheduleTextureUpdate();
public:
    ClientBlock(unsigned short x, unsigned short y, ClientMapBlock* block_data, ClientBlocks* parent_map) : x(x), y(y), block_data(block_data), parent_map(parent_map) {}
    void setType(BlockType block_id, LiquidType liquid_id);
    void setLightLevel(unsigned char level);
    void setBreakStage(unsigned char stage);
    void drawBack();
    void drawFront();
    void update();

    void updateOrientation();
    
    inline bool isGhost() { return block_data->getUniqueBlock().ghost; }
    inline unsigned char getLightLevel() { return block_data->light_level; }
    inline unsigned char getBreakStage() { return block_data->break_stage; }
    inline BlockType getType() { return block_data->block_id; }
    inline LiquidType getLiquidType() { return block_data->liquid_id; }
    inline void setLiquidLevel(unsigned char level) { block_data->liquid_level = level; }
    inline unsigned char getLiquidLevel() { return block_data->liquid_level; }
    inline float getSpeedMultiplier() { return block_data->getUniqueLiquid().speed_multiplier; }
    inline bool hasToUpdate() { return block_data->update; }
};

struct ClientMapChunk {
    ChunkState state = ChunkState::unloaded;
    bool update = true;
    gfx::Image back_texture, front_texture;
};

class ClientChunk {
    ClientMapChunk* chunk_data;
    unsigned short x, y;
    ClientBlocks* parent_map;

public:
    ClientChunk(unsigned short x, unsigned short y, ClientMapChunk* chunk_data, ClientBlocks* parent_map) : x(x), y(y), chunk_data(chunk_data), parent_map(parent_map) {}

    inline ChunkState getState() { return chunk_data->state; };
    inline void setState(ChunkState state) { chunk_data->state = state; }
    inline bool hasToUpdate() { return chunk_data->update; }
    inline void scheduleUpdate() { chunk_data->update = true; }

    void createTexture();
    void updateTexture();
    void drawBack();
    void drawFront();
};

class ClientBlocks : public gfx::GraphicalModule, EventListener<ClientPacketEvent> {
    unsigned short width{}, height{};
    ClientMapChunk *chunks = nullptr;
    ClientMapBlock *blocks = nullptr;

    networkingManager* networking_manager;
    
public:
    explicit ClientBlocks(networkingManager* manager, ResourcePack* resource_pack) : networking_manager(manager), resource_pack(resource_pack) {}
    int view_x{}, view_y{};
    
    ResourcePack* resource_pack;

    void onEvent(ClientPacketEvent& event) override;

    ClientChunk getChunk(unsigned short x, unsigned short y);
    ClientBlock getBlock(unsigned short x, unsigned short y);

    void renderBackBlocks();
    void renderFrontBlocks();
    
    inline unsigned short getWorldWidth() const { return width; }
    inline unsigned short getWorldHeight() const { return height; }

    void createWorld(unsigned short map_width, unsigned short map_height);

    ~ClientBlocks() override;
};

#endif
