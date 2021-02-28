//
//  blockEngine.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#ifndef blockEngine_hpp
#define blockEngine_hpp

#include "blockType.hpp"

#define BLOCK_WIDTH 16
#define MAX_LIGHT 100

namespace blockEngine {

struct block;

struct uniqueBlock {
    bool ghost, only_on_floor, transparent;
    
    std::string name;
    
    uniqueBlock(const std::string& name, bool ghost, bool only_on_floor, bool transparent, itemEngine::itemType drop, unsigned short break_time);
    
    void (*rightClickEvent)(block*, unsigned short, unsigned short) = nullptr;
    void (*leftClickEvent)(block*, unsigned short, unsigned short) = nullptr;
    
    itemEngine::itemType drop;
    unsigned short break_time;
};

struct block {
public:
    block() : block_id(AIR) {}
    explicit block(blockType block_id) : block_id(block_id) {}

    void update();
    
    blockType block_id;
    
    [[nodiscard]] uniqueBlock& getUniqueBlock() const;
    void setBlockType(blockType id);
    
    unsigned char light_level = 0;
    bool light_source = false;
    void light_update(bool update=true);
    
    bool to_update_light = true;
    unsigned short break_progress = 0;
    
    unsigned short getX() const;
    unsigned short getY() const;
};

struct chunk {
    bool loaded = false, pending_load = false;
};

void prepare();
void prepareWorld();
void close();

inline unsigned short world_width = 4400, world_height = 1200;
inline chunk *chunks;
inline block *blocks;

inline std::vector<uniqueBlock> unique_blocks;

block& getBlock(unsigned short x, unsigned short y);
chunk& getChunk(unsigned short x, unsigned short y);

void updateNeighbours(unsigned short x, unsigned short y);

void removeNaturalLight(unsigned short x);
void setNaturalLight(unsigned short x);

void addLightSource(unsigned short x, unsigned short y, unsigned char power);
void removeLightSource(unsigned short x, unsigned short y);

REGISTER_EVENT(block_change) {
    unsigned short x, y;
    blockEngine::blockType type;
};

REGISTER_EVENT(light_change) {
    unsigned short x, y;
};

}

#endif /* blockEngine_hpp */
