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

enum blockType {AIR, DIRT, STONE, GRASS_BLOCK};

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
};

struct unique_block {
    std::string name;
    SDL_Texture* texture;
    std::vector<blockType> connects_to;
    
    unique_block(std::string name);
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

inline std::vector<unique_block> block_types;
}

#endif /* blockEngine_h */
