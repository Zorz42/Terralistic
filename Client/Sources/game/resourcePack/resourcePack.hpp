#ifndef resourcePack_hpp
#define resourcePack_hpp

#include "properties.hpp"
#include "graphics.hpp"

class ResourcePack {
    gfx::Image block_textures[(int)BlockType::NUM_BLOCKS], item_textures[(int)ItemType::NUM_ITEMS], item_text_textures[(int)ItemType::NUM_ITEMS], liquid_textures[(int)LiquidType::NUM_LIQUIDS], breaking_texture, player_texture, background, texture_atlas;
public:
    void load(std::string path);
    
    const gfx::Image& getBlockTexture();
    const gfx::Image& getItemTexture(ItemType type);
    const gfx::Image& getItemTextTexture(ItemType type);
    const gfx::Image& getLiquidTexture(LiquidType type);
    const gfx::Image& getBreakingTexture();
    const gfx::Image& getPlayerTexture();
    const gfx::Image& getBackground();
};

#endif
