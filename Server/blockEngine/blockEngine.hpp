//
//  blockEngine.hpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifndef blockEngine_hpp
#define blockEngine_hpp

#include "networkingModule.hpp"
#include "playerHandler.hpp"

#define BLOCK_WIDTH 16

namespace blockEngine {

struct block;

struct uniqueBlock {
    bool ghost = false;
    bool only_on_floor = false;
    bool transparent = false;
    
    std::string name;
    
    uniqueBlock(const std::string& name, bool ghost, bool only_on_floor, bool transparent, itemEngine::itemType drop, unsigned short break_time);
    
    void (*rightClickEvent)(block*, unsigned short, unsigned short, playerHandler::player*) = nullptr;
    void (*leftClickEvent)(block*, unsigned short, unsigned short, playerHandler::player*) = nullptr;
    
    itemEngine::itemType drop;
    unsigned short break_time;
};

struct block {
    block() : block_id(AIR) {}
    explicit block(blockType block_id) : block_id(block_id) {}

    void update(unsigned short x, unsigned short y);
    
    blockType block_id;
    
    [[nodiscard]] uniqueBlock& getUniqueBlock() const;
    unsigned short break_progress = 0;
};

inline block *world;
inline unsigned short world_width;
inline unsigned short world_height;

block& getBlock(unsigned short x, unsigned short y);

inline std::vector<uniqueBlock> unique_blocks;

void updateNearestBlocks(unsigned short x, unsigned short y);
void blockChange(unsigned short x, unsigned short y, blockType type, networking::connection* conn=nullptr);

void updatePlayersBreaking();
}

#endif /* blockEngine_hpp */

