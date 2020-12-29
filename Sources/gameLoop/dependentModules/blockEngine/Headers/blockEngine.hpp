//
//  blockEngine.h
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#ifndef blockEngine_h
#define blockEngine_h

#include <SDL2/SDL.h>

#include <string>
#include <vector>
#include "itemType.hpp"
#include "blockType.hpp"

#define BLOCK_WIDTH 16

namespace blockEngine {

struct block;

struct uniqueBlock {
    bool single_texture = false;
    bool ghost = false;
    bool only_on_floor = false;
    bool transparent = false;
    
    std::string name;
    SDL_Texture* texture;
    std::vector<blockType> connects_to;
    
    uniqueBlock(const std::string& name, bool ghost, bool only_on_floor, bool transparent, itemEngine::itemType drop);
    
    void (*rightClickEvent)(block*) = nullptr;
    void (*leftClickEvent)(block*) = nullptr;
    
    itemEngine::itemType drop;
};

struct block {
    block() : block_id(AIR) {}
    explicit block(blockType block_id) : block_id(block_id) {}

    void draw();
    SDL_Rect getRect();
    unsigned short getX();
    unsigned short getY();
    void update();
    
    blockType block_id;
    Uint8 block_orientation{};
    
    [[nodiscard]] uniqueBlock& getUniqueBlock() const;
};

void init();
void prepare();
void close();

void render_blocks();

inline block *world;
inline unsigned short world_width;
inline unsigned short world_height;

block& getBlock(unsigned short x, unsigned short y);

inline int position_x, position_y;
inline int view_x, view_y;

inline std::vector<uniqueBlock> unique_blocks;

void rightClickEvent(unsigned short x, unsigned short y);
void leftClickEvent(unsigned short x, unsigned short y);

void updateNearestBlocks(unsigned short x, unsigned short y);

}

#endif /* blockEngine_h */
