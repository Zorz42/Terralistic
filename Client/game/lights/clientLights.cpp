#include "clientLights.hpp"

void ClientLights::init() {
#if DEVELOPER_MODE
    settings->addSetting(&light_enable_setting);
#endif
    Lights::init();
    light_color_change_event.addListener(this);
}

void ClientLights::postInit() {
    create();
    light_chunks = new LightChunk[getWidth() / 16 * getHeight() / 16];
    updateAllLightEmitters();
}

ClientLights::LightChunk* ClientLights::getLightChunk(int x, int y) {
    return &light_chunks[y * getWidth() / 16 + x];
}

void ClientLights::update(float frame_length) {
    enabled = light_enable_setting.getValue();
    
    bool finished = !enabled;
    while(!finished) {
        finished = true;
        for(int y = blocks->getBlocksExtendedViewBeginY(); y <= blocks->getBlocksExtendedViewEndY(); y++)
            for(int x = blocks->getBlocksExtendedViewBeginX(); x <= blocks->getBlocksExtendedViewEndX(); x++)
                if(hasScheduledLightUpdate(x, y)) {
                    updateLight(x, y);
                    finished = false;
                }
    }
}

void ClientLights::LightChunk::update(ClientLights* lights, int x, int y) {
    has_update = false;
    
    int index = 0;
    for(int x_ = x * CHUNK_SIZE; x_ < (x + 1) * CHUNK_SIZE; x_++)
        for(int y_ = y * CHUNK_SIZE; y_ < (y + 1) * CHUNK_SIZE; y_++) {
            int rel_x = x_ % CHUNK_SIZE, rel_y = y_ % CHUNK_SIZE;
            int x_stretch = x_ == 0 ? BLOCK_WIDTH : 0;
            int low_x = x_ == lights->getWidth() - 1 ? x_ : x_ + 1, low_y = y_ == lights->getHeight() - 1 ? y_ : y_ + 1;
            
            LightColor
            color1 = lights->getLightColor(x_, y_),
            color2 = lights->getLightColor(low_x, y_),
            color3 = lights->getLightColor(low_x, low_y),
            color4 = lights->getLightColor(x_, low_y);
            
            if(color1 != LightColor(255, 255, 255) || color2 != LightColor(255, 255, 255) || color3 != LightColor(255, 255, 255) || color4 != LightColor(255, 255, 255)) {
                light_rects.setRect(index, {rel_x * BLOCK_WIDTH * 2 + BLOCK_WIDTH - x_stretch, rel_y * BLOCK_WIDTH * 2 + BLOCK_WIDTH, BLOCK_WIDTH * 2 + x_stretch, BLOCK_WIDTH * 2});
                
                light_rects.setColor(index * 4, {
                    (unsigned char)(255.0 / MAX_LIGHT * color1.r),
                    (unsigned char)(255.0 / MAX_LIGHT * color1.g),
                    (unsigned char)(255.0 / MAX_LIGHT * color1.b),
                });
                light_rects.setColor(index * 4 + 1, {
                    (unsigned char)(255.0 / MAX_LIGHT * color2.r),
                    (unsigned char)(255.0 / MAX_LIGHT * color2.g),
                    (unsigned char)(255.0 / MAX_LIGHT * color2.b),
                });
                light_rects.setColor(index * 4 + 2, {
                    (unsigned char)(255.0 / MAX_LIGHT * color3.r),
                    (unsigned char)(255.0 / MAX_LIGHT * color3.g),
                    (unsigned char)(255.0 / MAX_LIGHT * color3.b),
                });
                light_rects.setColor(index * 4 + 3, {
                    (unsigned char)(255.0 / MAX_LIGHT * color4.r),
                    (unsigned char)(255.0 / MAX_LIGHT * color4.g),
                    (unsigned char)(255.0 / MAX_LIGHT * color4.b),
                });
                
                index++;
            }
        }
    
    lights_count = index;
}

void ClientLights::LightChunk::render(int x, int y) {
    light_rects.render(lights_count, nullptr, x, y, /*blend_multiply*/true);
}

void ClientLights::LightChunk::create(ClientLights *lights, int x, int y) {
    light_rects.resize(CHUNK_SIZE * CHUNK_SIZE);
    is_created = true;
}

void ClientLights::onEvent(LightColorChangeEvent& event) {
    scheduleLightUpdate(event.x, event.y);
    scheduleLightUpdate(event.x - 1, event.y);
    scheduleLightUpdate(event.x, event.y - 1);
    scheduleLightUpdate(event.x - 1, event.y - 1);
}

void ClientLights::render() {
    for(int x = blocks->getBlocksExtendedViewBeginX() / CHUNK_SIZE; x <= blocks->getBlocksExtendedViewEndX() / CHUNK_SIZE; x++)
        for(int y = blocks->getBlocksExtendedViewBeginY() / CHUNK_SIZE; y <= blocks->getBlocksExtendedViewEndY() / CHUNK_SIZE; y++) {
            if(!getLightChunk(x, y)->isCreated())
                getLightChunk(x, y)->create(this, x, y);
            
            if(getLightChunk(x, y)->has_update)
                getLightChunk(x, y)->update(this, x, y);
        }
    
    for(int x = blocks->getBlocksViewBeginX() / CHUNK_SIZE; x <= blocks->getBlocksViewEndX() / CHUNK_SIZE; x++)
        for(int y = blocks->getBlocksViewBeginY() / CHUNK_SIZE; y <= blocks->getBlocksViewEndY() / CHUNK_SIZE; y++)
            getLightChunk(x, y)->render(x * CHUNK_SIZE * BLOCK_WIDTH * 2 - camera->getX() + gfx::getWindowWidth() / 2, y * CHUNK_SIZE * BLOCK_WIDTH * 2 - camera->getY() + gfx::getWindowHeight() / 2);

}

void ClientLights::stop() {
#if DEVELOPER_MODE
    settings->removeSetting(&light_enable_setting);
#endif
    Lights::stop();
    light_color_change_event.removeListener(this);
    delete[] light_chunks;
}

void ClientLights::scheduleLightUpdate(int x, int y) {
    getLightChunk(x / CHUNK_SIZE, y / CHUNK_SIZE)->has_update = true;
}
