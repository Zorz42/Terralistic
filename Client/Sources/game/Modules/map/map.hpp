//
//  map.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/04/2021.
//

#ifndef map_hpp
#define map_hpp

#include <vector>
#include <string>

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include "networkingModule.hpp"

#define BLOCK_WIDTH 16
#define MAX_LIGHT 100

class map : public gfx::sceneModule, packetListener {
public:
    enum class blockType {AIR, DIRT, STONE_BLOCK, GRASS_BLOCK, STONE, WOOD, LEAVES};
    enum class chunkState {unloaded, pending_load, loaded};
    enum class itemType {NOTHING, STONE, DIRT, STONE_BLOCK};

    static void initBlocks();
    static void initItems();
    
protected:
    struct uniqueBlock {
        uniqueBlock(const std::string& name, bool ghost, std::vector<map::blockType> connects_to);
        
        bool ghost;
        std::string name;
        gfx::image texture;
        std::vector<map::blockType> connects_to;
        bool single_texture;
    };
    
    struct blockData {
        blockData(blockType block_id=blockType::AIR) : block_id(block_id) {}
        
        blockType block_id;
        unsigned char light_level = 0;
        unsigned char break_stage = 0;
        unsigned char orientation{0};
        bool update = true;
        
        uniqueBlock& getUniqueBlock() const;
    };
    
    struct chunkData {
        chunkState state = chunkState::unloaded;
        bool update = true;
        gfx::image texture;
    };
    
    static gfx::image breaking_texture;
    
    void renderBlocks();
    void renderItems();
    
    void render() override { renderBlocks(); renderItems(); }
    void onPacket(packets::packet packet) override;
    
    networkingManager* networking_manager;
    
public: // !!! should be protected
    struct uniqueItem {
        uniqueItem(const std::string& name, unsigned short stack_size, map::blockType places);
        std::string name;
        unsigned short stack_size;
        blockType places;
        gfx::image texture;
        gfx::sprite text_texture;
    };
    
protected:
public: // !!! should be protected
    static std::vector<uniqueItem> unique_items;
    static std::vector<uniqueBlock> unique_blocks;
    
public:
    class block {
        blockData* block_data = nullptr;
        unsigned short x, y;
        map* parent_map;
        
    public:
        block(unsigned short x, unsigned short y, blockData* block_data, map* parent_map) : x(x), y(y), block_data(block_data), parent_map(parent_map) {}
        block() = default;
        void setType(blockType id);
        void draw();
        void scheduleTextureUpdate();
        void update();
        
        inline bool isGhost() { return block_data->getUniqueBlock().ghost; }
        inline unsigned char getLightLevel() { return block_data->light_level; }
        inline unsigned char getBreakStage() { return block_data->break_stage; }
        inline blockType getType() { return block_data->block_id; }
        inline void setBreakStage(unsigned char stage) { block_data->break_stage = stage; }
        inline bool hasScheduledTextureUpdate() { return block_data->update; }
        inline bool refersToABlock() { return block_data != nullptr; }
        
        inline unsigned short getX() { return x; }
        inline unsigned short getY() { return y; }
        
        inline void setLightLevel(unsigned char level) { block_data->light_level = level; }
        void updateOrientation();
    };
    
    struct item {
    public:
        void create(itemType item_id_, int x_, int y_, unsigned short id_);
        int x, y;
        uniqueItem& getUniqueItem() const;
        unsigned short getId() { return id; }
        itemType getItemId() { return item_id; }
    protected:
        int velocity_x, velocity_y;
        unsigned short id;
        itemType item_id;
    };
    
    class chunk {
        chunkData* chunk_data;
        unsigned short x, y;
        map* parent_map;
        
    public:
        chunk(unsigned short x, unsigned short y, chunkData* chunk_data, map* parent_map) : x(x), y(y), chunk_data(chunk_data), parent_map(parent_map) {}
        
        inline chunkState getState() { return chunk_data->state; };
        inline void setState(chunkState state) { chunk_data->state = state; }
        inline bool hasToUpdate() { return chunk_data->update; }
        inline void scheduleUpdate() { chunk_data->update = true; }
        
        void createTexture();
        void updateTexture();
        void draw();
    };
    
protected:
    unsigned short width, height;
    chunkData *chunks = nullptr;
    blockData *blocks = nullptr;
    
    void onBlockChange(block& block);
    void onLightChange(block& block);
    void onBreakStageChange(block& block);
    
    std::vector<item> items;
    
public:
    map(networkingManager* manager) : packetListener(manager), networking_manager(manager) { listening_to = {packets::BLOCK_CHANGE, packets::CHUNK, packets::BLOCK_PROGRESS_CHANGE, packets::ITEM_CREATION, packets::ITEM_DELETION, packets::ITEM_MOVEMENT}; }
    int view_x, view_y;
    
    chunk getChunk(unsigned short x, unsigned short y);
    block getBlock(unsigned short x, unsigned short y);
    
    inline unsigned short getWorldWidth() { return width; }
    inline unsigned short getWorldHeight() { return height; }
    
    void createWorld(unsigned short width, unsigned short height);
    
    void spawnItem(itemType item_id, int x, int y, short id=-1);
    item* getItemById(unsigned short id);
    
    ~map();
};

#endif /* map_hpp */
