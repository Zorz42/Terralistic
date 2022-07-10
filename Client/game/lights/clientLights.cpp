#include "clientLights.hpp"

void ClientLights::init() {
#if DEVELOPER_MODE
    settings->addSetting(&light_enable_setting);
#endif
    Lights::init();
    light_color_change_event.addListener(this);
    debug_menu->registerDebugLine(&render_time_line);
}

void ClientLights::postInit() {
    create();
    light_chunks = new LightChunk[getWidth() / 16 * getHeight() / 16];
    updateAllLightEmitters();
    
    light_update_thread = std::thread(&ClientLights::lightUpdateLoop, this);
}

ClientLights::LightChunk* ClientLights::getLightChunk(int x, int y) {
    if(x < 0 || x >= getWidth() / 16 || y < 0 || y >= getHeight() / 16 || light_chunks == nullptr)
        throw Exception("Light chunk out of bounds.");
    return &light_chunks[y * getWidth() / 16 + x];
}

void ClientLights::update(float frame_length) {
    enabled = light_enable_setting.getValue();
}

void ClientLights::updateParallel(float frame_length) {
    for(int x = blocks->getBlocksExtendedViewBeginX() / CHUNK_SIZE; x <= blocks->getBlocksExtendedViewEndX() / CHUNK_SIZE; x++)
        for(int y = blocks->getBlocksExtendedViewBeginY() / CHUNK_SIZE; y <= blocks->getBlocksExtendedViewEndY() / CHUNK_SIZE; y++) {
            if(!getLightChunk(x, y)->isCreated())
                getLightChunk(x, y)->create();
            
            if(getLightChunk(x, y)->has_update)
                getLightChunk(x, y)->update(this, x, y);
        }
}

void ClientLights::lightUpdateLoop() {
    while(running) {
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
        gfx::sleep(5);
    }
}

void ClientLights::LightChunk::update(ClientLights* lights, int x, int y) {
    has_update = false;
    
    int index = 0;
    for(int y_ = y * CHUNK_SIZE; y_ < (y + 1) * CHUNK_SIZE; y_++)
        for(int x_ = x * CHUNK_SIZE; x_ < (x + 1) * CHUNK_SIZE; x_++) {
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
                
                light_rects.setColor(index, {
                    (unsigned char)(255.0 / MAX_LIGHT * color1.r),
                    (unsigned char)(255.0 / MAX_LIGHT * color1.g),
                    (unsigned char)(255.0 / MAX_LIGHT * color1.b),
                }, {
                    (unsigned char)(255.0 / MAX_LIGHT * color2.r),
                    (unsigned char)(255.0 / MAX_LIGHT * color2.g),
                    (unsigned char)(255.0 / MAX_LIGHT * color2.b),
                }, {
                    (unsigned char)(255.0 / MAX_LIGHT * color4.r),
                    (unsigned char)(255.0 / MAX_LIGHT * color4.g),
                    (unsigned char)(255.0 / MAX_LIGHT * color4.b),
                }, {
                    (unsigned char)(255.0 / MAX_LIGHT * color3.r),
                    (unsigned char)(255.0 / MAX_LIGHT * color3.g),
                    (unsigned char)(255.0 / MAX_LIGHT * color3.b),
                });
                
                index++;
            }
        }
    light_count = index;
}

void ClientLights::LightChunk::render(int x, int y) {
    if(light_count > 0)
        light_rects.render(nullptr, x, y, /*blend_multiply*/true, light_count);
}

void ClientLights::LightChunk::create() {
    light_rects.resize(CHUNK_SIZE * CHUNK_SIZE);
    is_created = true;
}

void ClientLights::onEvent(LightColorChangeEvent& event) {
    scheduleClientLightUpdate(event.x, event.y);
    scheduleClientLightUpdate(event.x - 1, event.y);
    scheduleClientLightUpdate(event.x, event.y - 1);
    scheduleClientLightUpdate(event.x - 1, event.y - 1);
}

void ClientLights::render() {
    gfx::Timer render_timer;
    for(int y = blocks->getBlocksViewBeginY() / CHUNK_SIZE; y <= blocks->getBlocksViewEndY() / CHUNK_SIZE; y++)
        for(int x = blocks->getBlocksViewBeginX() / CHUNK_SIZE; x <= blocks->getBlocksViewEndX() / CHUNK_SIZE; x++)
            if(getLightChunk(x, y)->isCreated())
                getLightChunk(x, y)->render(x * CHUNK_SIZE * BLOCK_WIDTH * 2 - camera->getX() + gfx::getWindowWidth() / 2, y * CHUNK_SIZE * BLOCK_WIDTH * 2 - camera->getY() + gfx::getWindowHeight() / 2);
    
    render_time_sum += render_timer.getTimeElapsed();
    fps_count++;
    if(line_refresh_timer.getTimeElapsed() >= 1000) {
        render_time_line.text = std::to_string(render_time_sum / fps_count) + "ms lights render";
        
        fps_count = 0;
        render_time_sum = 0;
        line_refresh_timer.reset();
    }
}

void ClientLights::stop() {
    running = false;
    light_update_thread.join();
#if DEVELOPER_MODE
    settings->removeSetting(&light_enable_setting);
#endif
    Lights::stop();
    light_color_change_event.removeListener(this);
    delete[] light_chunks;
}

void ClientLights::scheduleClientLightUpdate(int x, int y) {
    getLightChunk(x / CHUNK_SIZE, y / CHUNK_SIZE)->has_update = true;
}
