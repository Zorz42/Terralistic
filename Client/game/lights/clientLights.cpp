#include "clientLights.hpp"

void ClientLights::init() {
#if DEVELOPER_MODE
    settings->addSetting(&light_enable_setting);
#endif
    Lights::init();
    light_level_change_event.addListener(this);
}

void ClientLights::postInit() {
    create();
    light_chunks = new LightChunk[getWidth() / 16 * getHeight() / 16];
}

ClientLights::LightChunk* ClientLights::getLightChunk(int x, int y) {
    return &light_chunks[y * getWidth() / 16 + x];
}

void ClientLights::update(float frame_length) {
    enabled = light_enable_setting.getValue();
    
    bool finished = !enabled;
    while(!finished) {
        finished = true;
        for(int y = blocks->getViewBeginY(); y < blocks->getViewEndY(); y++)
            for(int x = blocks->getViewBeginX(); x < blocks->getViewEndX(); x++)
                if(hasScheduledLightUpdate(x, y)) {
                    updateLight(x, y);
                    finished = true;
                }
    }
}

void ClientLights::LightChunk::update(Lights* lights, int x, int y) {
    if(!is_created)
        return;
    
    int rel_x = x % LIGHT_CHUNK_SIZE, rel_y = y % LIGHT_CHUNK_SIZE;
    int index = LIGHT_CHUNK_SIZE * rel_y + rel_x;
    light_rects.setRect(index, {rel_x * BLOCK_WIDTH * 2 + BLOCK_WIDTH, rel_y * BLOCK_WIDTH * 2 + BLOCK_WIDTH, BLOCK_WIDTH * 2, BLOCK_WIDTH * 2});
    
    int low_x = x == lights->getWidth() - 1 ? x : x + 1, low_y = y == lights->getHeight() - 1 ? y : y + 1;
    light_rects.setColor(index * 4, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * lights->getLightLevel(x, y))});
    light_rects.setColor(index * 4 + 1, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * lights->getLightLevel(low_x, y))});
    light_rects.setColor(index * 4 + 2, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * lights->getLightLevel(low_x, low_y))});
    light_rects.setColor(index * 4 + 3, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * lights->getLightLevel(x, low_y))});
}

void ClientLights::LightChunk::render(int x, int y) {
    light_rects.render(LIGHT_CHUNK_SIZE * LIGHT_CHUNK_SIZE, nullptr, x, y);
}

void ClientLights::LightChunk::create(Lights *lights, int x, int y) {
    light_rects.resize(LIGHT_CHUNK_SIZE * LIGHT_CHUNK_SIZE);
    is_created = true;
    
    for(int x2 = 0; x2 < LIGHT_CHUNK_SIZE; x2++)
        for(int y2 = 0; y2 < LIGHT_CHUNK_SIZE; y2++) {
            light_rects.setRect(LIGHT_CHUNK_SIZE * y2 + x2, {x2 * BLOCK_WIDTH * 2 + BLOCK_WIDTH, y2 * BLOCK_WIDTH * 2 + BLOCK_WIDTH, BLOCK_WIDTH * 2, BLOCK_WIDTH * 2});
            update(lights, x * LIGHT_CHUNK_SIZE + x2, y * LIGHT_CHUNK_SIZE + y2);
        }
}

void ClientLights::onEvent(LightLevelChangeEvent& event) {
    int high_x = event.x == 0 ? event.x : event.x - 1, high_y = event.y == 0 ? event.y : event.y - 1;
    getLightChunk(event.x / LIGHT_CHUNK_SIZE, event.y / LIGHT_CHUNK_SIZE)->update(this, event.x, event.y);
    getLightChunk(high_x / LIGHT_CHUNK_SIZE, event.y / LIGHT_CHUNK_SIZE)->update(this, high_x, event.y);
    getLightChunk(event.x / LIGHT_CHUNK_SIZE, high_y / LIGHT_CHUNK_SIZE)->update(this, event.x, high_y);
    getLightChunk(high_x / LIGHT_CHUNK_SIZE, high_y / LIGHT_CHUNK_SIZE)->update(this, high_x, high_y);
}

void ClientLights::render() {
    for(int x = blocks->getViewBeginX() / 16; x <= blocks->getViewEndX() / 16; x++)
        for(int y = blocks->getViewBeginY() / 16; y <= blocks->getViewEndY() / 16; y++) {
            if(!getLightChunk(x, y)->isCreated())
                getLightChunk(x, y)->create(this, x, y);
            getLightChunk(x, y)->render(x * BLOCK_CHUNK_SIZE * BLOCK_WIDTH * 2 - blocks->view_x + gfx::getWindowWidth() / 2, y * BLOCK_CHUNK_SIZE * BLOCK_WIDTH * 2 - blocks->view_y + gfx::getWindowHeight() / 2);
        }
}

void ClientLights::stop() {
#if DEVELOPER_MODE
    settings->removeSetting(&light_enable_setting);
#endif
    Lights::stop();
    light_level_change_event.removeListener(this);
    delete[] light_chunks;
}
