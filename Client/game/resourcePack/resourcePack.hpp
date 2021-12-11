#pragma once
#include "clientModule.hpp"
#include "liquids.hpp"
#include "items.hpp"

class ResourcePack : public ClientModule {
    gfx::Texture *item_text_textures = nullptr, player_texture, background, liquid_texture_atlas, item_texture_atlas;
    gfx::RectShape *liquid_texture_rectangles = nullptr;
    gfx::RectShape *item_texture_rectangles = nullptr;
    std::vector<std::string> paths;
    
    void loadLiquids();
    void loadItems();
    void init() override;
    void stop() override;
    
    Liquids* liquids;
    Items* items;
public:
    ResourcePack(Liquids* liquids, Items* items) : liquids(liquids), items(items) {}

    std::string getFile(const std::string& file_name);

    const gfx::Texture& getItemTexture();
    const gfx::Texture& getItemTextTexture(ItemType* type);
    const gfx::Texture& getLiquidTexture();
    const gfx::Texture& getPlayerTexture();
    const gfx::Texture& getBackground();
    const gfx::RectShape& getTextureRectangle(LiquidType* type);
    const gfx::RectShape& getTextureRectangle(ItemType* type);
};
