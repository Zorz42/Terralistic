//
//  blockEngine.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#ifndef blockEngine_hpp
#define blockEngine_hpp

#include <SDL2/SDL.h>
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

    void draw(unsigned short x, unsigned short y) const;
    void update(unsigned short x, unsigned short y);
    
    blockType block_id;
    
    [[nodiscard]] uniqueBlock& getUniqueBlock() const;
    void setBlockType(blockType id, unsigned short x, unsigned short y);
    
    unsigned char light_level = 0;
    bool light_source = false;
    void light_update(unsigned short x, unsigned short y, bool update=true);
    
    bool to_update = true, to_update_light = true;
    unsigned short break_progress = 0;
private:
    Uint8 block_orientation{};
};

struct chunk {
    void render(unsigned short x, unsigned short y) const;
    block blocks[16][16];
    bool update = true, loaded = false, pending_load = false;
    SDL_Texture* texture = nullptr;
    void updateTexture();
    void createTexture();
};

void prepare();
void prepareWorld();
void close();

void render_blocks();

inline chunk *world;
inline unsigned short world_width = 4400, world_height = 1200;
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

inline SDL_Texture *breaking_texture = nullptr;

REGISTER_EVENT(block_change) {
    unsigned short x, y;
    blockEngine::blockType type;
};

}

#endif /* blockEngine_hpp */
