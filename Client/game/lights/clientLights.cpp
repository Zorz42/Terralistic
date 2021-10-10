#include "clientLights.hpp"

void ClientLights::init() {
    Lights::init();
}

void ClientLights::postInit() {
    create();
    for(int x = 0; x < getWidth(); x++)
        for(unsigned short y = 0; y < getHeight() && blocks->getBlockInfo(x, y).transparent; y++)
            setLightSource(x, y, MAX_LIGHT);
}

void ClientLights::update(float frame_length) {
    bool finished = false;
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

void ClientLights::render() {
    gfx::RectArray light_rects((blocks->getViewEndX() - blocks->getViewBeginX()) * (blocks->getViewEndY() - blocks->getViewBeginY()));
    
    int light_index = 0;
    for(unsigned short x = blocks->getViewBeginX(); x < blocks->getViewEndX(); x++)
        for(unsigned short y = blocks->getViewBeginY(); y < blocks->getViewEndY(); y++) {
            int block_x = x * BLOCK_WIDTH * 2 - blocks->view_x + gfx::getWindowWidth() / 2, block_y = y * BLOCK_WIDTH * 2 - blocks->view_y + gfx::getWindowHeight() / 2;
            
            unsigned short low_x = x + 1 == blocks->getWidth() ? x : x + 1, low_y = y + 1 == blocks->getHeight() ? y : y + 1;
            unsigned char light_levels[] = {getLightLevel(x, y), getLightLevel(low_x, y), getLightLevel(low_x, low_y), getLightLevel(x, low_y)};
            
            if(light_levels[0] != MAX_LIGHT || light_levels[1] != MAX_LIGHT || light_levels[2] != MAX_LIGHT || light_levels[3] != MAX_LIGHT) {
                light_rects.setColor(light_index * 4, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getLightLevel(x, y))});
                light_rects.setColor(light_index * 4 + 1, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getLightLevel(low_x, y))});
                light_rects.setColor(light_index * 4 + 2, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getLightLevel(low_x, low_y))});
                light_rects.setColor(light_index * 4 + 3, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getLightLevel(x, low_y))});
                
                light_rects.setRect(light_index, {short(block_x + BLOCK_WIDTH), short(block_y + BLOCK_WIDTH), (short)BLOCK_WIDTH * 2, (short)BLOCK_WIDTH * 2});
                
                light_index++;
            }
        }
    
    light_rects.resize(light_index);
    if(light_index)
        light_rects.render();
}

void ClientLights::stop() {
    Lights::stop();
}
