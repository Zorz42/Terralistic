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
    light_updates = new bool[getWidth() * getHeight()];
    for(int i = 0; i < getWidth() * getHeight(); i++)
        light_updates[i] = false;
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
        for(int y = blocks->getBlocksViewBeginY(); y <= blocks->getBlocksViewEndY(); y++)
            for(int x = blocks->getBlocksViewBeginX(); x <= blocks->getBlocksViewEndX(); x++)
                if(hasScheduledLightUpdate(x, y)) {
                    updateLight(x, y);
                    finished = false;
                }
    }
}

void ClientLights::LightChunk::update(ClientLights* lights, int x, int y) {
    if(!is_created)
        return;
    
    int rel_x = x % CHUNK_SIZE, rel_y = y % CHUNK_SIZE;
    int index = CHUNK_SIZE * rel_y + rel_x;
    
    int low_x = x == lights->getWidth() - 1 ? x : x + 1, low_y = y == lights->getHeight() - 1 ? y : y + 1;
    
    LightColor
    color1 = lights->getLightColor(x, y),
    color2 = lights->getLightColor(low_x, y),
    color3 = lights->getLightColor(low_x, low_y),
    color4 = lights->getLightColor(x, low_y);
    
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
}

void ClientLights::LightChunk::render(int x, int y) {
    light_rects.render(CHUNK_SIZE * CHUNK_SIZE, nullptr, x, y, /*blend_multiply*/true);
}

void ClientLights::LightChunk::create(ClientLights *lights, int x, int y) {
    light_rects.resize(CHUNK_SIZE * CHUNK_SIZE);
    is_created = true;
    
    for(int x2 = 0; x2 < CHUNK_SIZE; x2++)
        for(int y2 = 0; y2 < CHUNK_SIZE; y2++) {
            int x_stretch = (x == 0 && x2 == 0) ? BLOCK_WIDTH : 0;
            light_rects.setRect(CHUNK_SIZE * y2 + x2, {x2 * BLOCK_WIDTH * 2 + BLOCK_WIDTH - x_stretch, y2 * BLOCK_WIDTH * 2 + BLOCK_WIDTH, BLOCK_WIDTH * 2 + x_stretch, BLOCK_WIDTH * 2});
            update(lights, x * CHUNK_SIZE + x2, y * CHUNK_SIZE + y2);
        }
}

void ClientLights::onEvent(LightColorChangeEvent& event) {
    getLightUpdate(event.x, event.y) = true;
}

void ClientLights::render() {
    for(int x = blocks->getBlocksViewBeginX(); x <= blocks->getBlocksViewEndX() && x < getWidth() - 1; x++)
        for(int y = blocks->getBlocksViewBeginY(); y <= blocks->getBlocksViewEndY() && y < getHeight() - 1; y++)
            if(getLightUpdate(x, y) || getLightUpdate(x + 1, y) || getLightUpdate(x, y + 1) || getLightUpdate(x + 1, y + 1))
                getLightChunk(x / CHUNK_SIZE, y / CHUNK_SIZE)->update(this, x, y);
    
    for(int x = blocks->getBlocksViewBeginX(); x <= blocks->getBlocksViewEndX() + 1 && x < getWidth(); x++)
        for(int y = blocks->getBlocksViewBeginY(); y <= blocks->getBlocksViewEndY() + 1 && y < getHeight(); y++)
            getLightUpdate(x, y) = false;
    
    for(int x = blocks->getBlocksViewBeginX() / CHUNK_SIZE; x <= blocks->getBlocksViewEndX() / CHUNK_SIZE; x++)
        for(int y = blocks->getBlocksViewBeginY() / CHUNK_SIZE; y <= blocks->getBlocksViewEndY() / CHUNK_SIZE; y++) {
            if(!getLightChunk(x, y)->isCreated())
                getLightChunk(x, y)->create(this, x, y);
            
            getLightChunk(x, y)->render(x * CHUNK_SIZE * BLOCK_WIDTH * 2 - camera->getX() + gfx::getWindowWidth() / 2, y * CHUNK_SIZE * BLOCK_WIDTH * 2 - camera->getY() + gfx::getWindowHeight() / 2);
        }
}

void ClientLights::stop() {
#if DEVELOPER_MODE
    settings->removeSetting(&light_enable_setting);
#endif
    Lights::stop();
    light_color_change_event.removeListener(this);
    delete[] light_chunks;
}

bool& ClientLights::getLightUpdate(int x, int y) {
    if(x < 0 || x >= getWidth() || y < 0 || y >= getHeight())
        throw Exception("Light update is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
    return light_updates[y * getWidth() + x];
}
