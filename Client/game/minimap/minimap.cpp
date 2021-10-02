#include "minimap.hpp"

#define MINIMAP_SIZE 200
#define MINIMAP_SCALE 1

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
    
    gfx::PixelGrid block_pixels(MINIMAP_SIZE, MINIMAP_SIZE), liquid_pixels(MINIMAP_SIZE, MINIMAP_SIZE), light_pixels(MINIMAP_SIZE, MINIMAP_SIZE);
    
    for(int y = 0; y < MINIMAP_SIZE; y++)
        for(int x = 0; x < MINIMAP_SIZE; x++) {
            int block_x = client_blocks->view_x / BLOCK_WIDTH / 2 - MINIMAP_SIZE / 2 + x;
            int block_y = client_blocks->view_y / BLOCK_WIDTH / 2 - MINIMAP_SIZE / 2 + y;
            
            gfx::Color block_color = TRANSPARENT, liquid_color = TRANSPARENT;
            unsigned char light_level = MAX_LIGHT;
            
            if(block_x >= 0 && block_y >= 0 && block_x < blocks->getWidth() && block_y < blocks->getHeight()) {
                block_color = block_colors[(int)blocks->getBlockType(block_x, block_y)];
                liquid_color = liquid_colors[(int)blocks->getBlockType(block_x, block_y)];
                light_level = lights->getLightLevel(block_x, block_y);
                
                if(lights->hasScheduledLightUpdate(block_x, block_y))
                    lights->updateLight(block_x, block_y);
            }
            
            block_pixels.setPixel(x, y, block_color);
            liquid_pixels.setPixel(x, y, liquid_color);
            light_pixels.setPixel(x, y, {0, 0, 0, (unsigned char)((MAX_LIGHT - (int)light_level) * 255 / MAX_LIGHT)});
        }
    
    gfx::Sprite minimap_sprite;
    minimap_sprite.orientation = gfx::TOP_RIGHT;
    minimap_sprite.x = -SPACING / 2;
    minimap_sprite.y = SPACING / 2;
    minimap_sprite.scale = MINIMAP_SCALE;
    
    minimap_sprite.loadFromPixelGrid(block_pixels);
    minimap_sprite.render();
    
    minimap_sprite.loadFromPixelGrid(liquid_pixels);
    minimap_sprite.render();
    
    minimap_sprite.loadFromPixelGrid(light_pixels);
    minimap_sprite.render();
}
