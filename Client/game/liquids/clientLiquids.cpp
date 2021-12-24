#include "clientLiquids.hpp"

void ClientLiquids::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case ServerPacketType::LIQUID: {
            int x, y;
            int liquid_type;
            float liquid_level;
            event.packet >> x >> y >> liquid_type >> liquid_level;
            
            setLiquidType(x, y, getLiquidTypeById(liquid_type));
            setLiquidLevel(x, y, liquid_level);
            
            break;
        }
        default:;
    }
}

void ClientLiquids::onEvent(WelcomePacketEvent& event) {
    if(event.packet_type == WelcomePacketType::LIQUIDS)
        loadFromSerial(networking->getData());
}

void ClientLiquids::onEvent(LiquidChangeEvent& event) {
    getLiquidChunk(event.x / 16, event.y / 16)->update(this, event.x, event.y);
}

void ClientLiquids::init() {
    networking->packet_event.addListener(this);
    networking->welcome_packet_event.addListener(this);
    liquid_change_event.addListener(this);
    
    std::vector<gfx::Texture*> liquid_textures(getNumLiquidTypes() - 1);

    for(int i = 1; i < getNumLiquidTypes(); i++) {
        liquid_textures[i - 1] = new gfx::Texture;
        liquid_textures[i - 1]->loadFromFile(resource_pack->getFile("/liquids/" + getLiquidTypeById(i)->name + ".png"));
    }
    
    liquids_atlas.create(liquid_textures);
    
    for(int i = 1; i < getNumLiquidTypes(); i++)
        delete liquid_textures[i - 1];
}

void ClientLiquids::postInit() {
    liquid_chunks = new LiquidChunk[getWidth() / 16 * getHeight() / 16];
}

void ClientLiquids::stop() {
    networking->packet_event.removeListener(this);
    networking->welcome_packet_event.removeListener(this);
    liquid_change_event.removeListener(this);
    delete[] liquid_chunks;
}

void ClientLiquids::render() {
    for(int x = blocks->getBlocksViewBeginX() / LIQUID_CHUNK_SIZE; x <= blocks->getBlocksViewEndX() / LIQUID_CHUNK_SIZE; x++)
        for(int y = blocks->getBlocksViewBeginY() / LIQUID_CHUNK_SIZE; y <= blocks->getBlocksViewEndY() / LIQUID_CHUNK_SIZE; y++) {
            if(!getLiquidChunk(x, y)->isCreated())
                getLiquidChunk(x, y)->create(this, x, y);
            
            getLiquidChunk(x, y)->render(this, x * LIQUID_CHUNK_SIZE * BLOCK_WIDTH * 2 - camera->getX() + gfx::getWindowWidth() / 2, y * LIQUID_CHUNK_SIZE * BLOCK_WIDTH * 2 - camera->getY() + gfx::getWindowHeight() / 2);
        }
}

void ClientLiquids::LiquidChunk::create(ClientLiquids* liquids, int x, int y) {
    liquid_rects.resize(LIQUID_CHUNK_SIZE * LIQUID_CHUNK_SIZE);
    is_created = true;
    
    for(int x2 = 0; x2 < LIQUID_CHUNK_SIZE; x2++)
        for(int y2 = 0; y2 < LIQUID_CHUNK_SIZE; y2++)
            update(liquids, x * LIQUID_CHUNK_SIZE + x2, y * LIQUID_CHUNK_SIZE + y2);
}

void ClientLiquids::LiquidChunk::update(ClientLiquids* liquids, int x, int y) {
    if(!is_created)
        return;
    
    int rel_x = x % LIQUID_CHUNK_SIZE, rel_y = y % LIQUID_CHUNK_SIZE;
    int index = LIQUID_CHUNK_SIZE * rel_y + rel_x;
    if(liquids->getLiquidType(x, y) != &liquids->empty) {
        int texture_y = liquids->getLiquidRectInAtlas(liquids->getLiquidType(x, y)).y * 2;
        liquid_rects.setTextureCoords(index, {0, texture_y, BLOCK_WIDTH, BLOCK_WIDTH});
        
        int level = liquids->getLiquidLevel(x, y) / MAX_LIQUID_LEVEL * BLOCK_WIDTH * 2;
        liquid_rects.setRect(index, {rel_x * BLOCK_WIDTH * 2, rel_y * BLOCK_WIDTH * 2 + BLOCK_WIDTH * 2 - level, BLOCK_WIDTH * 2, level});
    } else {
        liquid_rects.setTextureCoords(index, {0, 0, 0, 0});
        liquid_rects.setRect(index, {0, 0, 0, 0});
    }
}

void ClientLiquids::LiquidChunk::render(ClientLiquids* liquids, int x, int y) {
    liquid_rects.render(16 * 16, &liquids->getLiquidsAtlasTexture(), x, y);
}

const gfx::Texture& ClientLiquids::getLiquidsAtlasTexture() {
    return liquids_atlas.getTexture();
}

gfx::RectShape ClientLiquids::getLiquidRectInAtlas(LiquidType* type) {
    return liquids_atlas.getRect(type->id - 1);
}

ClientLiquids::LiquidChunk* ClientLiquids::getLiquidChunk(int x, int y) {
    return &liquid_chunks[y * getWidth() / 16 + x];
}
