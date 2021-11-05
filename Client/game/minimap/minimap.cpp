#include "minimap.hpp"

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
    
    minimap_texture.create(MINIMAP_SIZE, MINIMAP_SIZE);
    
    settings->addSetting(&minimap_toggle_setting);
}

void Minimap::stop() {
    settings->removeSetting(&minimap_toggle_setting);
}

void Minimap::update(float frame_length) {
    enabled = minimap_toggle_setting.getValue();
}

void Minimap::render() {
    back_rect.render();
    
    for(int y = 0; y < MINIMAP_SIZE; y++)
        for(int x = 0; x < MINIMAP_SIZE; x++) {
            int block_x = blocks->view_x / BLOCK_WIDTH / 2 - MINIMAP_SIZE / 2 + x;
            int block_y = blocks->view_y / BLOCK_WIDTH / 2 - MINIMAP_SIZE / 2 + y;
            
            gfx::Color block_color = TRANSPARENT, liquid_color = TRANSPARENT;
            unsigned char light_level = MAX_LIGHT;
            
            if(block_x >= 0 && block_y >= 0 && block_x < blocks->getWidth() && block_y < blocks->getHeight()) {
                block_color = blocks->getBlockType(block_x, block_y)->color;
                liquid_color = blocks->getBlockType(block_x, block_y)->color;
                light_level = lights->getLightLevel(block_x, block_y);
                
                if(lights->hasScheduledLightUpdate(block_x, block_y))
                    lights->updateLight(block_x, block_y);
            }
            
            block_pixels.setPixel(x, y, block_color);
            liquid_pixels.setPixel(x, y, liquid_color);
            light_pixels.setPixel(x, y, {0, 0, 0, (unsigned char)((MAX_LIGHT - (int)light_level) * 255 / MAX_LIGHT)});
        }
    
    for(int x = 0; x < MINIMAP_SIZE; x++) {
        int translated_x = blocks->view_x / BLOCK_WIDTH / 2 - MINIMAP_SIZE / 2 + x;
        if(translated_x >= 0 && translated_x < blocks->getWidth())
            natural_light->updateLight(translated_x);
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
