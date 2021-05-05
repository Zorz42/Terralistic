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

#ifdef DEVELOPER_MODE
#include <Graphics_Debug/graphics.hpp>
#else
#include <Graphics/graphics.hpp>
#endif

#endif

#include "networkingModule.hpp"

#define BLOCK_WIDTH 16
#define MAX_LIGHT 100

class map : public gfx::sceneModule, packetListener {
public:
    enum class blockType {AIR, DIRT, STONE_BLOCK, GRASS_BLOCK, STONE, WOOD, LEAVES, TOTAL_BLOCKS};
    enum class chunkState {unloaded, pending_load, loaded};
    enum class itemType {NOTHING, STONE, DIRT, STONE_BLOCK, TOTAL_ITEMS};
    
protected:
    struct uniqueBlock {
        uniqueBlock(const std::string& name, bool ghost, std::vector<map::blockType> connects_to);
        uniqueBlock() = default;
        
        bool ghost, single_texture;
        std::string name;
        gfx::image texture;
        std::vector<map::blockType> connects_to;
    };
    
    struct blockData {
        blockData(blockType block_id=blockType::AIR) : block_id(block_id) {}
        
        blockType block_id;
        unsigned char light_level = 0, break_stage = 0, orientation = 0;
        bool update = true;
        
        uniqueBlock& getUniqueBlock() const;
    };
    
    struct chunkData {
        chunkState state = chunkState::unloaded;
        bool update = true;
        gfx::image texture;
    };
    
    void renderBlocks();
    void renderItems();
    
    void render() override { renderBlocks(); renderItems(); }
    void onPacket(packets::packet packet) override;
    
    networkingManager* networking_manager;
    
public: // !!! should be protected
    struct uniqueItem {
        uniqueItem(const std::string& name, unsigned short stack_size);
        uniqueItem() = default;
        
        std::string name;
        unsigned short stack_size;
        gfx::image texture;
        
        void draw(short x, short y, unsigned char scale);
    };
    
    static uniqueItem* unique_items;
    static uniqueBlock* unique_blocks;
    
public:
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
    
    class block {
        friend chunk;
        blockData* block_data;
        unsigned short x, y;
        map* parent_map;
        
        void scheduleTextureUpdate();
        void updateOrientation();
        
    public:
        block(unsigned short x, unsigned short y, blockData* block_data, map* parent_map) : x(x), y(y), block_data(block_data), parent_map(parent_map) {}
        void setType(blockType id);
        void setLightLevel(unsigned char level);
        void setBreakStage(unsigned char stage);
        void draw();
        void update();
        
        inline unsigned short getX() { return x; }
        inline unsigned short getY() { return y; }
        
        inline bool isGhost() { return block_data->getUniqueBlock().ghost; }
        inline unsigned char getLightLevel() { return block_data->light_level; }
        inline unsigned char getBreakStage() { return block_data->break_stage; }
        inline blockType getType() { return block_data->block_id; }
    };
    
    class item {
        uniqueItem& getUniqueItem() const;
        unsigned short id;
        itemType item_type;
    public:
        item(itemType item_type, int x, int y, unsigned short id) : x(x * 100), y(y * 100), id(id), item_type(item_type) {}
        int x, y;
        unsigned short getId() { return id; }
        itemType getType() { return item_type; }
        void draw(short x, short y, unsigned char scale);
    };
    
protected:
    unsigned short width, height;
    chunkData *chunks = nullptr;
    blockData *blocks = nullptr;
    
    std::vector<item> items;
    
public:
    map(networkingManager* manager) : packetListener(manager), networking_manager(manager) { listening_to = {packets::BLOCK_CHANGE, packets::CHUNK, packets::BLOCK_PROGRESS_CHANGE, packets::ITEM_CREATION, packets::ITEM_DELETION, packets::ITEM_MOVEMENT, packets::LIGHT_CHANGE}; }
    int view_x, view_y;
    
    static void initBlocks();
    static void initItems();
    
    chunk getChunk(unsigned short x, unsigned short y);
    block getBlock(unsigned short x, unsigned short y);
    
    inline unsigned short getWorldWidth() { return width; }
    inline unsigned short getWorldHeight() { return height; }
    
    void createWorld(unsigned short map_width, unsigned short map_height);
    
    item* getItemById(unsigned short id);
    
    ~map();
};

#endif /* map_hpp */
