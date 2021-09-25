#ifndef minimap_hpp
#define minimap_hpp

#include "graphics.hpp"
#include "clientBlocks.hpp"

class Minimap : public gfx::SceneModule {
    gfx::Rect back_rect;
    ClientBlocks* blocks;
    gfx::Color block_colors[(int)BlockType::NUM_BLOCKS];
    gfx::Color liquid_colors[(int)LiquidType::NUM_LIQUIDS];
    sf::Texture minimap_texture;
    
    void init() override;
    void render() override;
    void onKeyDown(gfx::Key key) override;
public:
    Minimap(ClientBlocks* blocks) : blocks(blocks) {}
};

#endif
