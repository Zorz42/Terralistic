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
#define MAX_LIGHT 100

namespace blockEngine {

struct block;

struct uniqueBlock {
    bool single_texture, ghost, only_on_floor, transparent;
    
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

    void draw(unsigned short x, unsigned short y) const;
    void update(unsigned short x, unsigned short y);
    
    blockType block_id;
    Uint8 block_orientation{};
    
    [[nodiscard]] uniqueBlock& getUniqueBlock() const;
    void setBlockType(blockType id, unsigned short x, unsigned short y);
    
    unsigned char light_level = 0;
    bool light_source = false;
    void light_update(unsigned short x, unsigned short y, bool update=true);
    
    bool to_update = true, to_update_light = true;
};

struct chunk {
    void render(unsigned short x, unsigned short y) const;
    block blocks[16][16];
    bool update = true, loaded = false, pending_load = false;
    SDL_Texture* texture = nullptr;
    void updateTexture();
    void createTexture();
};

void init();
void prepare();
void prepareWorld();
void close();

void render_blocks();

inline chunk *world;
inline unsigned short world_width, world_height;
inline std::vector<uniqueBlock> unique_blocks;

block& getBlock(unsigned short x, unsigned short y);
chunk& getChunk(unsigned short x, unsigned short y);

void rightClickEvent(unsigned short x, unsigned short y);
void leftClickEvent(unsigned short x, unsigned short y);

void updateNearestBlocks(unsigned short x, unsigned short y);

void handleEvents(SDL_Event& event);

void removeNaturalLight(unsigned short x);
void setNaturalLight(unsigned short x);

void addLightSource(unsigned short x, unsigned short y, unsigned char power);
void removeLightSource(unsigned short x, unsigned short y);

}

#endif /* blockEngine_h */
