#include "clientBlocks.hpp"
#include "readOpa.hpp"
#include "resourcePath.hpp"

#define EXTENDED_VIEW_MARGIN 100

void ClientBlocks::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case ServerPacketType::BLOCK: {
            int x, y;
            unsigned char block_id;
            unsigned char x_from_main, y_from_main;
            event.packet >> x >> y >> block_id >> x_from_main >> y_from_main;
            
            setBlockType(x, y, getBlockTypeById(block_id), x_from_main, y_from_main);
            break;
        }
        case ServerPacketType::BLOCK_STARTED_BREAKING: {
            int x, y;
            event.packet >> x >> y;
            startBreakingBlock(x, y);
            break;
        }
        case ServerPacketType::BLOCK_STOPPED_BREAKING: {
            int x, y;
            event.packet >> x >> y;
            stopBreakingBlock(x, y);
            break;
        }
        default:;
    }
}

ClientBlocks::RenderBlock* ClientBlocks::getRenderBlock(int x, int y) {
    if(x < 0 || x >= getWidth() || y < 0 || y >= getHeight() || render_blocks == nullptr)
        throw Exception("RenderBlock is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
    return &render_blocks[y * getWidth() + x];
}

ClientBlocks::RenderBlockChunk* ClientBlocks::getRenderBlockChunk(int x, int y) {
    if(x < 0 || x >= getWidth() / CHUNK_SIZE || y < 0 || y >= getHeight() / CHUNK_SIZE || block_chunks == nullptr)
        throw Exception("Chunk is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
    return &block_chunks[y * getWidth() / CHUNK_SIZE + x];
}

void ClientBlocks::scheduleBlockUpdate(int x, int y) {
    getRenderBlockChunk(x / CHUNK_SIZE, y / CHUNK_SIZE)->has_update = true;
}

void ClientBlocks::updateState(int x, int y) {
    setState(x, y, getBlockType(x, y)->updateState(this, x, y));
}

void ClientBlocks::setState(int x, int y, int state) {
    getRenderBlock(x, y)->state = state;
}

int ClientBlocks::getState(int x, int y) {
    return getRenderBlock(x, y)->state;
}

void ClientBlocks::postInit() {
    render_blocks = new RenderBlock[getWidth() * getHeight()];
    block_chunks = new RenderBlockChunk[getWidth() / 16 * getHeight() / 16];
}

void ClientBlocks::onEvent(BlockChangeEvent& event) {
    int coords[5][2] = {{event.x, event.y}, {event.x + 1, event.y}, {event.x - 1, event.y}, {event.x, event.y + 1}, {event.x, event.y - 1}};
    for(auto & coord : coords) {
        if(!(coord[0] < 0 || coord[1] < 0 || coord[0] >= getWidth() || coord[1] >= getHeight())) {
            scheduleBlockUpdate(coord[0], coord[1]);
            updateState(coord[0], coord[1]);
        }
    }
}

void ClientBlocks::onEvent(WelcomePacketEvent& event) {
    if(event.packet_type == WelcomePacketType::BLOCKS) {
        std::vector<char> data;
        event.packet >> data;
        fromSerial(data);
    }
}

void ClientBlocks::init() {
    block_change_event.addListener(this);
    networking->packet_event.addListener(this);
    networking->welcome_packet_event.addListener(this);
}

void ClientBlocks::loadTextures() {
    loadOpa(breaking_texture, resource_pack->getFile("/misc/breaking.opa"));
    
    std::vector<gfx::Texture*> block_textures(getNumBlockTypes() - 1);

    for(int i = 1; i < getNumBlockTypes(); i++) {
        block_textures[i - 1] = new gfx::Texture;
        loadOpa(*block_textures[i - 1], resource_pack->getFile("/blocks/" + getBlockTypeById(i)->name + ".opa"));
    }
    
    blocks_atlas.create(block_textures);
    
    for(int i = 1; i < getNumBlockTypes(); i++)
        delete block_textures[i - 1];
}

void ClientBlocks::updateParallel(float frame_length) {
    updateBreakingBlocks(frame_length);
    
    view_begin_x = std::max(camera->getViewBeginX() / (BLOCK_WIDTH * 2) - 2, 0);
    view_end_x = std::min(camera->getViewEndX() / (BLOCK_WIDTH * 2) + 2, getWidth() - 1);

    view_begin_y = std::max(camera->getViewBeginY() / (BLOCK_WIDTH * 2) - 2, 0);
    view_end_y = std::min(camera->getViewEndY() / (BLOCK_WIDTH * 2) + 2, getHeight() - 1);

    extended_view_begin_x = std::max(camera->getViewBeginX() / (BLOCK_WIDTH * 2) - EXTENDED_VIEW_MARGIN, 0);
    extended_view_end_x = std::min(camera->getViewEndX() / (BLOCK_WIDTH * 2) + EXTENDED_VIEW_MARGIN, getWidth() - 1);

    extended_view_begin_y = std::max(camera->getViewBeginY() / (BLOCK_WIDTH * 2) - EXTENDED_VIEW_MARGIN, 0);
    extended_view_end_y = std::min(camera->getViewEndY() / (BLOCK_WIDTH * 2) + EXTENDED_VIEW_MARGIN, getHeight() - 1);
    
    for(int x = getBlocksViewBeginX() / CHUNK_SIZE; x <= getBlocksViewEndX() / CHUNK_SIZE; x++)
        for(int y = getBlocksViewBeginY() / CHUNK_SIZE; y <= getBlocksViewEndY() / CHUNK_SIZE; y++) {
            if(!getRenderBlockChunk(x, y)->isCreated())
                getRenderBlockChunk(x, y)->create();
            
            if(getRenderBlockChunk(x, y)->has_update)
                getRenderBlockChunk(x, y)->update(this, x, y);
        }
}

void ClientBlocks::stop() {
    block_change_event.removeListener(this);
    networking->packet_event.removeListener(this);
    networking->welcome_packet_event.removeListener(this);
    
    delete[] render_blocks;
    delete[] block_chunks;
}

void ClientBlocks::RenderBlockChunk::create() {
    block_rects.resize(CHUNK_SIZE * CHUNK_SIZE);
    is_created = true;
}

void ClientBlocks::RenderBlockChunk::update(ClientBlocks* blocks, int x, int y) {
    has_update = false;
    int index = 0;
    for(int x_ = x * CHUNK_SIZE; x_ < (x + 1) * CHUNK_SIZE; x_++)
        for(int y_ = y * CHUNK_SIZE; y_ < (y + 1) * CHUNK_SIZE; y_++)
            if(blocks->getBlockType(x_, y_) != &blocks->air) {
                if(blocks->getState(x_, y_) == 16)
                    blocks->updateState(x_, y_);
                block_rects.setRect(index, {(x_ % CHUNK_SIZE) * BLOCK_WIDTH * 2, (y_ % CHUNK_SIZE) * BLOCK_WIDTH * 2, BLOCK_WIDTH * 2, BLOCK_WIDTH * 2});
                
                int texture_x;
                int texture_y;
                if(blocks->getBlockType(x_, y_)->width == 0) {
                    texture_x = std::abs(blocks->getRenderBlock(x_, y_)->variation) % (blocks->getBlockRectInAtlas(blocks->getBlockType(x_, y_)).w / BLOCK_WIDTH) * BLOCK_WIDTH;
                    texture_y = blocks->getBlockRectInAtlas(blocks->getBlockType(x_, y_)).y + BLOCK_WIDTH * blocks->getRenderBlock(x_, y_)->state;
                } else {
                    texture_x = blocks->getBlockRectInAtlas(blocks->getBlockType(x_, y_)).x + blocks->getBlockXFromMain(x_, y_) * BLOCK_WIDTH;
                    texture_y = blocks->getBlockRectInAtlas(blocks->getBlockType(x_, y_)).y + blocks->getBlockYFromMain(x_, y_) * BLOCK_WIDTH;
                }
                block_rects.setTextureCoords(index, {texture_x, texture_y, BLOCK_WIDTH, BLOCK_WIDTH});
                block_rects.setColor(index, {255, 255, 255});
                index++;
            }
    block_count = index;
}

void ClientBlocks::RenderBlockChunk::render(ClientBlocks* blocks, int x, int y) {
    if(block_count > 0)
        block_rects.render(&blocks->getBlocksAtlasTexture(), x, y);
}

const gfx::Texture& ClientBlocks::getBlocksAtlasTexture() {
    return blocks_atlas.getTexture();
}

gfx::RectShape ClientBlocks::getBlockRectInAtlas(BlockType* type) {
    return blocks_atlas.getRect(type->id - 1);
}

void ClientBlocks::render() {
    for(int x = getBlocksViewBeginX() / CHUNK_SIZE; x <= getBlocksViewEndX() / CHUNK_SIZE; x++)
        for(int y = getBlocksViewBeginY() / CHUNK_SIZE; y <= getBlocksViewEndY() / CHUNK_SIZE; y++)
            if(getRenderBlockChunk(x, y)->isCreated()) {
                getRenderBlockChunk(x, y)->render(this, x * CHUNK_SIZE * BLOCK_WIDTH * 2 - camera->getX() + gfx::getWindowWidth() / 2, y * CHUNK_SIZE * BLOCK_WIDTH * 2 - camera->getY() + gfx::getWindowHeight() / 2);
                
                if(getChunkBreakingBlocksCount(x, y) > 0) {
                    for(int x_ = x * CHUNK_SIZE; x_ < (x + 1) * CHUNK_SIZE; x_++)
                        for(int y_ = y * CHUNK_SIZE; y_ < (y + 1) * CHUNK_SIZE; y_++)
                            if(getBreakStage(x_, y_)) {
                                int block_x = x_ * BLOCK_WIDTH * 2 - camera->getX() + gfx::getWindowWidth() / 2, block_y = y_ * BLOCK_WIDTH * 2 - camera->getY() + gfx::getWindowHeight() / 2;
                                breaking_texture.render(2, block_x, block_y, gfx::RectShape(0, BLOCK_WIDTH * (getBreakStage(x_, y_) - 1), BLOCK_WIDTH, BLOCK_WIDTH));
                            }
                }
            }
}

int ClientBlocks::getBlocksViewBeginX() const {
    return view_begin_x;
}

int ClientBlocks::getBlocksViewEndX() const {
    return view_end_x;
}

int ClientBlocks::getBlocksViewBeginY() const {
    return view_begin_y;
}

int ClientBlocks::getBlocksViewEndY() const {
    return view_end_y;
}

int ClientBlocks::getBlocksExtendedViewBeginX() const {
    return extended_view_begin_x;
}

int ClientBlocks::getBlocksExtendedViewEndX() const {
    return extended_view_end_x;
}

int ClientBlocks::getBlocksExtendedViewBeginY() const {
    return extended_view_begin_y;
}

int ClientBlocks::getBlocksExtendedViewEndY() const {
    return extended_view_end_y;
}

