#pragma once
#include "clientModule.hpp"
#include "liquids.hpp"
#include "items.hpp"

class ResourcePack : public ClientModule {
    gfx::Texture *item_text_textures = nullptr, breaking_texture, player_texture, background, block_texture_atlas, liquid_texture_atlas, item_texture_atlas;
    gfx::RectShape *block_texture_rectangles = nullptr;
    gfx::RectShape *liquid_texture_rectangles = nullptr;
    gfx::RectShape *item_texture_rectangles = nullptr;
    std::string getFile(const std::string& file_name);
    std::vector<std::string> paths;
    
    void loadBlocks();
    void loadLiquids();
    void loadItems();
    void init() override;
    void stop() override;
    
    Blocks* blocks;
    Liquids* liquids;
    Items* items;
public:
    ResourcePack(Blocks* blocks, Liquids* liquids, Items* items) : blocks(blocks), liquids(liquids), items(items) {}

    const gfx::Texture& getBlockTexture();
    const gfx::Texture& getItemTexture();
    const gfx::Texture& getItemTextTexture(ItemType* type);
    const gfx::Texture& getLiquidTexture();
    const gfx::Texture& getBreakingTexture();
    const gfx::Texture& getPlayerTexture();
    const gfx::Texture& getBackground();
    const gfx::RectShape& getTextureRectangle(BlockType* type);
    const gfx::RectShape& getTextureRectangle(LiquidType* type);
    const gfx::RectShape& getTextureRectangle(ItemType* type);
};
