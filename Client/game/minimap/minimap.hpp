#pragma once

#include "graphics.hpp"
#include "clientBlocks.hpp"
#include "liquids.hpp"
#include "lights.hpp"
#include "naturalLight.hpp"
#include "settings.hpp"

#define MINIMAP_SIZE 100
#define MINIMAP_SCALE 2

class Minimap : public ClientModule {
    gfx::Rect back_rect;
    gfx::PixelGrid block_pixels, liquid_pixels, light_pixels;
    Liquids* liquids;
    Lights* lights;
    ClientBlocks* blocks;
    NaturalLight* natural_light;
    Settings* settings;
    sf::Texture minimap_texture;
    BooleanSetting minimap_toggle_setting;
    
    void init() override;
    void stop() override;
    void update(float frame_length) override;
    void render() override;
public:
    Minimap(Settings* settings, ClientBlocks* blocks, Liquids* liquids, Lights* lights, NaturalLight* natural_light) : settings(settings), blocks(blocks), liquids(liquids), lights(lights), natural_light(natural_light), block_pixels(MINIMAP_SIZE, MINIMAP_SIZE), liquid_pixels(MINIMAP_SIZE, MINIMAP_SIZE), light_pixels(MINIMAP_SIZE, MINIMAP_SIZE), minimap_toggle_setting("Minimap", true) {}
};
