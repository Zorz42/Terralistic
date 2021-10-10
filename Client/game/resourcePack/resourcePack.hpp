#pragma once

#include "properties.hpp"
#include "graphics.hpp"
#include "clientModule.hpp"

class ResourcePack : public ClientModule {
    gfx::Texture item_text_textures[(int)ItemType::NUM_ITEMS], breaking_texture, player_texture, background, block_texture_atlas, liquid_texture_atlas, item_texture_atlas;
    gfx::RectShape block_texture_rectangles[(int)BlockType::NUM_BLOCKS];
    gfx::RectShape liquid_texture_rectangles[(int)LiquidType::NUM_LIQUIDS];
    std::string getFile(const std::string& file_name);
    std::vector<std::string> paths;
    
    gfx::RectShape item_texture_rectangles[(int)BlockType::NUM_BLOCKS];
    
    void loadBlocks();
    void loadLiquids();
    void loadItems();
    void init() override;
public:

    const gfx::Texture& getBlockTexture();
    const gfx::Texture& getItemTexture();
    const gfx::Texture& getItemTextTexture(ItemType type);
    const gfx::Texture& getLiquidTexture();
    const gfx::Texture& getBreakingTexture();
    const gfx::Texture& getPlayerTexture();
    const gfx::Texture& getBackground();
    const gfx::RectShape& getTextureRectangle(BlockType type);
    const gfx::RectShape& getTextureRectangle(LiquidType type);
    const gfx::RectShape& getTextureRectangle(ItemType type);

};
