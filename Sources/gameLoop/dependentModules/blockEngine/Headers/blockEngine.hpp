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

#define BLOCK_WIDTH 16

namespace blockEngine {

enum blockType {AIR, DIRT, STONE_BLOCK, GRASS_BLOCK, STONE};

struct block;

struct unique_block {
    bool single_texture = false;
    bool ghost = false;
    bool only_on_floor = false;
    bool transparent = false;
    
    std::string name;
    SDL_Texture* texture;
    std::vector<blockType> connects_to;
    
    unique_block(std::string name, bool ghost, bool only_on_floor, bool transparent);
    
    void (*rightClickEvent)(block*) = nullptr;
    void (*leftClickEvent)(block*) = nullptr;
};

struct block {
    block() : block_id(AIR) {}
    block(blockType block_id) : block_id(block_id) {}
    
    void draw();
    SDL_Rect getRect();
    unsigned int getX();
    unsigned int getY();
    void update();
    
    blockType block_id;
    Uint8 block_orientation;
    
    unique_block& getUniqueBlock();
};

void init();
void prepare();
void close();

void render_blocks();

inline block *world;
inline unsigned int world_width;
inline unsigned int world_height;

block& getBlock(unsigned int x, unsigned int y);

inline long position_x, position_y;
inline long view_x, view_y;

inline std::vector<unique_block> unique_blocks;

void rightClickEvent(int x, int y);
void leftClickEvent(int x, int y);

void updateNearestBlocks(int x, int y);

}

#endif /* blockEngine_h */
