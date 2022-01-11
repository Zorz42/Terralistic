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
                    finished = true;
                }
    }
}

LightColor ClientLights::getAverageColor(int x, int y) {
    int r = 0, g = 0, b = 0;
    int lights[4][2] = {{x + 1, y}, {x, y + 1}, {x - 1, y}, {x, y - 1}};
    int count = 0;
    for(int i = 0; i < 4; i++)
        if(lights[i][0] >= 0 && lights[i][0] < getWidth() && lights[i][1] >= 0 && lights[i][1] < getHeight()) {
            LightColor color = getLightColor(lights[i][0], lights[i][1]);
            r += color.r;
            g += color.g;
            b += color.b;
            count++;
        }
    r /= count;
    g /= count;
    b /= count;
    return LightColor(r, g, b);
}

void ClientLights::LightChunk::update(ClientLights* lights, int x, int y) {
    if(!is_created)
        return;
    
    int rel_x = x % LIGHT_CHUNK_SIZE, rel_y = y % LIGHT_CHUNK_SIZE;
    int index = LIGHT_CHUNK_SIZE * rel_y + rel_x;
    light_rects.setRect(index, {rel_x * BLOCK_WIDTH * 2 + BLOCK_WIDTH, rel_y * BLOCK_WIDTH * 2 + BLOCK_WIDTH, BLOCK_WIDTH * 2, BLOCK_WIDTH * 2});
    
    int low_x = x == lights->getWidth() - 1 ? x : x + 1, low_y = y == lights->getHeight() - 1 ? y : y + 1;
    
    LightColor
    color1 = lights->isLightSource(x, y) ? lights->getAverageColor(x, y) : lights->getLightColor(x, y),
    color2 = lights->isLightSource(low_x, y) ? lights->getAverageColor(low_x, y) : lights->getLightColor(low_x, y),
    color3 = lights->isLightSource(low_x, low_y) ? lights->getAverageColor(low_x, low_y) : lights->getLightColor(low_x, low_y),
    color4 = lights->isLightSource(x, low_y) ? lights->getAverageColor(x, low_y) : lights->getLightColor(x, low_y);
    
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
    light_rects.render(LIGHT_CHUNK_SIZE * LIGHT_CHUNK_SIZE, nullptr, x, y, /*blend_multiply*/true);
}

void ClientLights::LightChunk::create(ClientLights *lights, int x, int y) {
    light_rects.resize(LIGHT_CHUNK_SIZE * LIGHT_CHUNK_SIZE);
    is_created = true;
    
    for(int x2 = 0; x2 < LIGHT_CHUNK_SIZE; x2++)
        for(int y2 = 0; y2 < LIGHT_CHUNK_SIZE; y2++) {
            light_rects.setRect(LIGHT_CHUNK_SIZE * y2 + x2, {x2 * BLOCK_WIDTH * 2 + BLOCK_WIDTH, y2 * BLOCK_WIDTH * 2 + BLOCK_WIDTH, BLOCK_WIDTH * 2, BLOCK_WIDTH * 2});
            update(lights, x * LIGHT_CHUNK_SIZE + x2, y * LIGHT_CHUNK_SIZE + y2);
        }
}

void ClientLights::onEvent(LightColorChangeEvent& event) {
    for(int x = event.x - 2; x <= event.x + 2; x++)
        for(int y = event.y - 2; y <= event.y + 2; y++)
            if(x >= 0 && x < getWidth() && y >= 0 && y < getHeight())
                getLightChunk(x / LIGHT_CHUNK_SIZE, y / LIGHT_CHUNK_SIZE)->update(this, x, y);
}

void ClientLights::render() {
    for(int x = blocks->getBlocksViewBeginX() / LIGHT_CHUNK_SIZE; x <= blocks->getBlocksViewEndX() / LIGHT_CHUNK_SIZE; x++)
        for(int y = blocks->getBlocksViewBeginY() / LIGHT_CHUNK_SIZE; y <= blocks->getBlocksViewEndY() / LIGHT_CHUNK_SIZE; y++) {
            if(!getLightChunk(x, y)->isCreated())
                getLightChunk(x, y)->create(this, x, y);
            getLightChunk(x, y)->render(x * BLOCK_CHUNK_SIZE * BLOCK_WIDTH * 2 - camera->getX() + gfx::getWindowWidth() / 2, y * BLOCK_CHUNK_SIZE * BLOCK_WIDTH * 2 - camera->getY() + gfx::getWindowHeight() / 2);
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
