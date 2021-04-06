//
//  map.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/04/2021.
//

#ifndef map_hpp
#define map_hpp

#include <vector>

#define BLOCK_WIDTH 16
#define MAX_LIGHT 100

class map {
public:
    enum class blockType {AIR, DIRT, STONE_BLOCK, GRASS_BLOCK, STONE, WOOD, LEAVES};
    enum class chunkState {unloaded, pending_load, loaded};
    enum class itemType {NOTHING, STONE, DIRT, STONE_BLOCK};

public: // !!! should be private
    struct uniqueItem {
        uniqueItem(std::string  name, unsigned short stack_size, map::blockType places);
        std::string name;
        unsigned short stack_size;
        map::blockType places;
    };
    
    struct uniqueBlock {
        uniqueBlock(std::string  name, bool ghost, bool only_on_floor, bool transparent, itemType drop, unsigned short break_time) : ghost(ghost), only_on_floor(only_on_floor), transparent(transparent), name(std::move(name)), drop(drop), break_time(break_time) {}
        
        bool ghost, only_on_floor, transparent;
        std::string name;
        itemType drop;
        unsigned short break_time;
    };
    
private:
    struct blockData {
        blockData(blockType block_id=blockType::AIR) : block_id(block_id) {}
        
        blockType block_id;
        unsigned char light_level = 0;
        bool light_source = false, to_update_light = true;
        unsigned short break_progress = 0;
        unsigned char break_stage = 0;
        
        uniqueBlock& getUniqueBlock() const;
    };
public: // !!! should be private
    static std::vector<uniqueBlock> unique_blocks;
    static std::vector<uniqueItem> unique_items;
    
public:
    class block {
        blockData* block_data;
        unsigned short x, y;
        map* parent_map;
    public:
        block(unsigned short x, unsigned short y, blockData* block_data, map* parent_map) : x(x), y(y), block_data(block_data), parent_map(parent_map) {}
        void update();
        void setType(blockType id);
        void breakBlock();
        void setBreakProgress(unsigned short ms);
        void lightUpdate(bool update=true);
        void setLightSource(unsigned char power);
        void removeLightSource();
        
        inline bool isTransparent() { return block_data->getUniqueBlock().transparent; }
        inline bool isOnlyOnFloor() { return block_data->getUniqueBlock().only_on_floor; }
        inline bool isLightSource() { return block_data->light_source; }
        inline unsigned short getBreakTime() { return block_data->getUniqueBlock().break_time; }
        inline unsigned char getLightLevel() { return block_data->light_level; }
        inline unsigned short getBreakProgress() { return block_data->break_progress; }
        inline unsigned char getBreakStage() { return block_data->break_stage; }
        inline itemType getDrop() { return block_data->getUniqueBlock().drop; }
        inline void scheduleLightUpdate() { block_data->to_update_light = true; }
        inline blockType getType() { return block_data->block_id; }
    };
    
    struct item {
    public:
        void create(itemType item_id_, int x_, int y_, unsigned short id_);
        void destroy();
        int x, y;
        void update(float frame_length);
        bool colliding() const;
        uniqueItem& getUniqueItem() const;
        unsigned short getId() { return id; }
        itemType getItemId() { return item_id; }
    private:
        int velocity_x, velocity_y;
        unsigned short id;
        itemType item_id;
    };
    
private:
    unsigned short width = 4400, height = 1200;
    chunkState *chunk_states = nullptr;
    blockData *blocks = nullptr;
    std::vector<item> items;
    
    void removeNaturalLight(unsigned short x);
    void setNaturalLight(unsigned short x);
public:
    chunkState& getChunkState(unsigned short x, unsigned short y);
    block getBlock(unsigned short x, unsigned short y);
    
    int getSpawnX();
    int getSpawnY();
    
    inline unsigned short getWorldWidth() { return width; }
    inline unsigned short getWorldHeight() { return height; }
    
    void createWorld(unsigned short width, unsigned short height);
    
    void setNaturalLight();
    
    void spawnItem(itemType item_id, int x, int y, short id=-1);
    
    item* getItemById(unsigned short id);
    
    ~map();
};

#endif /* map_hpp */
