#pragma once

#include "properties.hpp"
#include "graphics.hpp"
#include "clientModule.hpp"

class ResourcePack : public ClientModule {
    gfx::Texture item_text_textures[(int)ItemTypeOld::NUM_ITEMS], breaking_texture, player_texture, background, block_texture_atlas, liquid_texture_atlas, item_texture_atlas;
    gfx::RectShape block_texture_rectangles[(int)BlockTypeOld::NUM_BLOCKS];
    gfx::RectShape liquid_texture_rectangles[(int)LiquidTypeOld::NUM_LIQUIDS];
    std::string getFile(const std::string& file_name);
    std::vector<std::string> paths;
    
    gfx::RectShape item_texture_rectangles[(int)BlockTypeOld::NUM_BLOCKS];
    
    void loadBlocks();
    void loadLiquids();
    void loadItems();
    void init() override;
public:

    const gfx::Texture& getBlockTexture();
    const gfx::Texture& getItemTexture();
    const gfx::Texture& getItemTextTexture(ItemTypeOld type);
    const gfx::Texture& getLiquidTexture();
    const gfx::Texture& getBreakingTexture();
    const gfx::Texture& getPlayerTexture();
    const gfx::Texture& getBackground();
    const gfx::RectShape& getTextureRectangle(BlockTypeOld type);
    const gfx::RectShape& getTextureRectangle(LiquidTypeOld type);
    const gfx::RectShape& getTextureRectangle(ItemTypeOld type);

};
