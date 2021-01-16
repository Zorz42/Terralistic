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
#include "lightingEngine.hpp"

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
    
    void (*rightClickEvent)(block*, unsigned short, unsigned short) = nullptr;
    void (*leftClickEvent)(block*, unsigned short, unsigned short) = nullptr;
    
    itemEngine::itemType drop;
};

struct block {
    block() : block_id(AIR) {}
    explicit block(blockType block_id) : block_id(block_id) {}

    void draw(unsigned short x, unsigned short y);
    SDL_Rect getRect(unsigned short x, unsigned short y);
    void update(unsigned short x, unsigned short y);
    
    blockType block_id;
    Uint8 block_orientation{};
    
    [[nodiscard]] uniqueBlock& getUniqueBlock() const;
    void setBlockType(blockType id, unsigned short x, unsigned short y, bool send_packet=true);
};

struct chunk {
    void render(unsigned short x, unsigned short y);
    block blocks[16][16];
    lightingEngine::lightBlock light_blocks[16][16];
    bool updates[16][16], update = true;
    SDL_Texture* texture = nullptr;
    void updateTexture();
    void createTexture();
};

void init();
void prepare();
void prepareChunks();
void close();

void render_blocks();

inline chunk *world;
inline unsigned short world_width;
inline unsigned short world_height;

block& getBlock(unsigned short x, unsigned short y);
chunk& getChunk(unsigned short x, unsigned short y);
void setUpdateBlock(unsigned short x, unsigned short y, bool value);

inline int position_x, position_y;
inline int view_x, view_y;

inline std::vector<uniqueBlock> unique_blocks;

void rightClickEvent(unsigned short x, unsigned short y);
void leftClickEvent(unsigned short x, unsigned short y);

void updateNearestBlocks(unsigned short x, unsigned short y);

}

#endif /* blockEngine_h */
