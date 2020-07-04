//
//  blockEngine.h
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#ifndef blockEngine_h
#define blockEngine_h

#include <SDL2/SDL.h>
#include <iostream>
#include <vector>

namespace block_engine {

enum blockType {BLOCK_AIR, BLOCK_DIRT};

struct block {
    block(unsigned short block_id) : block_id(block_id) {}
    
    void draw(unsigned int x, unsigned int y);
    unsigned short block_id;
};

struct unique_block {
    std::string name;
    SDL_Texture* texture;
    
    unique_block(std::string name) : name(name) {}
};

void init();

void render_blocks();

inline std::vector<block> world;
inline unsigned int world_width;
unsigned int getWorldHeight();

block& getBlock(unsigned int x, unsigned int y);

inline long position_x, position_y;

inline std::vector<unique_block> block_types;
}

#endif /* blockEngine_h */
