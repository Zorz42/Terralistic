#ifndef minimap_hpp
#define minimap_hpp

#include "graphics.hpp"
#include "clientBlocks.hpp"
#include "liquids.hpp"
#include "lights.hpp"

class Minimap : public gfx::SceneModule {
    gfx::Rect back_rect;
    Liquids* liquids;
    Lights* lights;
    ClientBlocks* blocks;
    gfx::Color block_colors[(int)BlockType::NUM_BLOCKS];
    gfx::Color liquid_colors[(int)LiquidType::NUM_LIQUIDS];
    sf::Texture minimap_texture;
    
    void init() override;
    void render() override;
public:
    Minimap(ClientBlocks* blocks, Liquids* liquids, Lights* lights) : blocks(blocks), liquids(liquids), lights(lights) {}
};

#endif
