#include "minimap.hpp"

#define MINIMAP_SIZE 100
#define MINIMAP_SCALE 2

void Minimap::init() {
    back_rect.orientation = gfx::TOP_RIGHT;
    back_rect.setWidth(MINIMAP_SIZE * MINIMAP_SCALE);
    back_rect.setHeight(MINIMAP_SIZE * MINIMAP_SCALE);
    back_rect.setX(-SPACING / 2);
    back_rect.setY(SPACING / 2);
    back_rect.fill_color.a = TRANSPARENCY;
    back_rect.blur_intensity = BLUR;
    back_rect.shadow_intensity = SHADOW_INTENSITY;
    back_rect.border_color = GREY;
    
    block_colors[(int)BlockType::AIR] = {0, 0, 0, 0};
    block_colors[(int)BlockType::DIRT] = {115, 77, 38};
    block_colors[(int)BlockType::STONE_BLOCK] = {128, 128, 128, 255};
    block_colors[(int)BlockType::GRASS_BLOCK] = {0, 153, 0, 255};
    block_colors[(int)BlockType::STONE] = {128, 128, 128, 255};
    block_colors[(int)BlockType::WOOD] = {128, 85, 0, 255};
    block_colors[(int)BlockType::LEAVES] = {0, 179, 0, 255};
    block_colors[(int)BlockType::SAND] = {210, 170, 109, 255};
    block_colors[(int)BlockType::SNOWY_GRASS_BLOCK] = {217, 217, 217, 255};
    block_colors[(int)BlockType::SNOW_BLOCK] = {242, 242, 242, 255};
    block_colors[(int)BlockType::ICE] = {179, 217, 255, 255};
    
    liquid_colors[(int)LiquidType::EMPTY] = {0, 0, 0, 0};
    liquid_colors[(int)LiquidType::WATER] = {0, 92, 230, 150};
    
    minimap_texture.create(MINIMAP_SIZE, MINIMAP_SIZE);
}

void Minimap::render() {
    back_rect.render();
    
    sf::Uint8 *block_pixels = new sf::Uint8[MINIMAP_SIZE * MINIMAP_SIZE * 4], *liquid_pixels = new sf::Uint8[MINIMAP_SIZE * MINIMAP_SIZE * 4], *light_pixels = new sf::Uint8[MINIMAP_SIZE * MINIMAP_SIZE * 4];
    
    int i = 0;
    for(int y = 0; y < MINIMAP_SIZE; y++)
        for(int x = 0; x < MINIMAP_SIZE; x++) {
            int block_x = blocks->view_x / BLOCK_WIDTH / 2 - MINIMAP_SIZE / 2 + x;
            int block_y = blocks->view_y / BLOCK_WIDTH / 2 - MINIMAP_SIZE / 2 + y;
            
            gfx::Color block_color = TRANSPARENT, liquid_color = TRANSPARENT;
            unsigned char light_level = MAX_LIGHT;
            
            if(block_x >= 0 && block_y >= 0 && block_x < blocks->getWidth() && block_y < blocks->getHeight()) {
                block_color = block_colors[(int)blocks->getBlock(block_x, block_y).getBlockType()];
                liquid_color = liquid_colors[(int)blocks->getBlock(block_x, block_y).getLiquidType()];
                light_level = blocks->getBlock(block_x, block_y).getLightLevel();
            }
            
            block_pixels[i] = block_color.r;
            block_pixels[i + 1] = block_color.g;
            block_pixels[i + 2] = block_color.b;
            block_pixels[i + 3] = block_color.a;
            
            liquid_pixels[i] = liquid_color.r;
            liquid_pixels[i + 1] = liquid_color.g;
            liquid_pixels[i + 2] = liquid_color.b;
            liquid_pixels[i + 3] = liquid_color.a;
            
            light_pixels[i] = 0;
            light_pixels[i + 1] = 0;
            light_pixels[i + 2] = 0;
            light_pixels[i + 3] = (MAX_LIGHT - (int)light_level) * 255 / MAX_LIGHT;
            i += 4;
        }
    sf::Sprite sprite;
    sprite.setPosition(back_rect.getTranslatedX(), back_rect.getTranslatedY());
    sprite.scale(MINIMAP_SCALE, MINIMAP_SCALE);
    
    minimap_texture.update(block_pixels);
    sprite.setTexture(minimap_texture);
    gfx::drawSFMLSprite(sprite);
    
    minimap_texture.update(liquid_pixels);
    sprite.setTexture(minimap_texture);
    gfx::drawSFMLSprite(sprite);
    
    minimap_texture.update(light_pixels);
    sprite.setTexture(minimap_texture);
    gfx::drawSFMLSprite(sprite);
}

void Minimap::onKeyDown(gfx::Key key) {
    
}
