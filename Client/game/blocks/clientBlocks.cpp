#include "clientBlocks.hpp"

#define EXTENDED_VIEW_MARGIN 100

bool ClientBlocks::updateOrientationSide(int x, int y, int side_x, int side_y) {
    return x + side_x >= getWidth() || x + side_x < 0 || y + side_y >= getHeight() || y + side_y < 0 ||
    getBlockType(x + side_x, y + side_y) == getBlockType(x, y) ||
    std::count(getBlockType(x, y)->connects_to.begin(), getBlockType(x, y)->connects_to.end(), getBlockType(x + side_x, y + side_y));
}

void ClientBlocks::updateOrientationDown(int x, int y) {
    setState(x, y, getState(x, y) * 2);
    if(updateOrientationSide(x, y, 0, 1))
        setState(x, y, getState(x, y) + 1);
}

void ClientBlocks::updateOrientationUp(int x, int y) {
    setState(x, y, getState(x, y) * 2);
    if(updateOrientationSide(x, y, 0, -1))
        setState(x, y, getState(x, y) + 1);
}

void ClientBlocks::updateOrientationLeft(int x, int y) {
    setState(x, y, getState(x, y) * 2);
    if(updateOrientationSide(x, y, -1, 0))
        setState(x, y, getState(x, y) + 1);
}

void ClientBlocks::updateOrientationRight(int x, int y) {
    setState(x, y, getState(x, y) * 2);
    if(updateOrientationSide(x, y, 1, 0))
        setState(x, y, getState(x, y) + 1);
}

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
    if(x < 0 || x >= getWidth() || y < 0 || y >= getHeight())
        throw Exception("RenderBlock is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
    return &render_blocks[y * getWidth() + x];
}

ClientBlocks::RenderBlockChunk* ClientBlocks::getRenderBlockChunk(int x, int y) {
    if(x < 0 || x >= getWidth() / CHUNK_SIZE || y < 0 || y >= getHeight() / CHUNK_SIZE)
        throw Exception("Chunk is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
    return &block_chunks[y * getWidth() / CHUNK_SIZE + x];
}

void ClientBlocks::scheduleBlockUpdate(int x, int y) {
    getRenderBlockChunk(x / CHUNK_SIZE, y / CHUNK_SIZE)->has_update = true;
}

void ClientBlocks::updateState(int x, int y) {
    getRenderBlock(x, y)->state = 0;
    if(getBlockType(x, y) != &air && getBlockRectInAtlas(getBlockType(x, y)).h != 8) {
        updateOrientationLeft(x, y);
        updateOrientationDown(x, y);
        updateOrientationRight(x, y);
        updateOrientationUp(x, y);
    }
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
    for(int i = 0; i < 5; i++) {
        scheduleBlockUpdate(coords[i][0], coords[i][1]);
        updateState(coords[i][0], coords[i][1]);
    }
}

void ClientBlocks::onEvent(WelcomePacketEvent& event) {
    if(event.packet_type == WelcomePacketType::BLOCKS)
        fromSerial(event.data);
}

void ClientBlocks::init() {
    block_change_event.addListener(this);
    networking->packet_event.addListener(this);
    networking->welcome_packet_event.addListener(this);
}

void ClientBlocks::loadTextures() {
    breaking_texture.loadFromFile(resource_pack->getFile("/misc/breaking.png"));
    
    std::vector<gfx::Texture*> block_textures(getNumBlockTypes() - 1);

    for(int i = 1; i < getNumBlockTypes(); i++) {
        block_textures[i - 1] = new gfx::Texture;
        block_textures[i - 1]->loadFromFile(resource_pack->getFile("/blocks/" + getBlockTypeById(i)->name + ".png"));
    }
    
    blocks_atlas.create(block_textures);
    
    for(int i = 1; i < getNumBlockTypes(); i++)
        delete block_textures[i - 1];
}

void ClientBlocks::update(float frame_length) {
    updateBreakingBlocks(frame_length);
    
    view_begin_x = std::max(camera->getViewBeginX() / (BLOCK_WIDTH * 2) - 2, 0);
    view_end_x = std::min(camera->getViewEndX() / (BLOCK_WIDTH * 2) + 2, getWidth() - 1);

    view_begin_y = std::max(camera->getViewBeginY() / (BLOCK_WIDTH * 2) - 2, 0);
    view_end_y = std::min(camera->getViewEndY() / (BLOCK_WIDTH * 2) + 2, getHeight() - 1);

    extended_view_begin_x = std::max(camera->getViewBeginX() / (BLOCK_WIDTH * 2) - EXTENDED_VIEW_MARGIN, 0);
    extended_view_end_x = std::min(camera->getViewEndX() / (BLOCK_WIDTH * 2) + EXTENDED_VIEW_MARGIN, getWidth() - 1);

    extended_view_begin_y = std::max(camera->getViewBeginY() / (BLOCK_WIDTH * 2) - EXTENDED_VIEW_MARGIN, 0);
    extended_view_end_y = std::min(camera->getViewEndY() / (BLOCK_WIDTH * 2) + EXTENDED_VIEW_MARGIN, getHeight() - 1);
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
                    texture_x = (blocks->getRenderBlock(x_, y_)->variation) % (blocks->getBlockRectInAtlas(blocks->getBlockType(x_, y_)).w / BLOCK_WIDTH) * BLOCK_WIDTH;
                    texture_y = blocks->getBlockRectInAtlas(blocks->getBlockType(x_, y_)).y + BLOCK_WIDTH * blocks->getRenderBlock(x_, y_)->state;
                } else {
                    texture_x = blocks->getBlockRectInAtlas(blocks->getBlockType(x_, y_)).x + blocks->getBlockXFromMain(x_, y_) * BLOCK_WIDTH;
                    texture_y =blocks->getBlockRectInAtlas(blocks->getBlockType(x_, y_)).y + blocks->getBlockYFromMain(x_, y_) * BLOCK_WIDTH;
                }
                block_rects.setTextureCoords(index, {texture_x, texture_y, BLOCK_WIDTH, BLOCK_WIDTH});
                index++;
            }
    block_count = index;
}

void ClientBlocks::RenderBlockChunk::render(ClientBlocks* blocks, int x, int y) {
    block_rects.render(block_count, &blocks->getBlocksAtlasTexture(), x, y);
}

const gfx::Texture& ClientBlocks::getBlocksAtlasTexture() {
    return blocks_atlas.getTexture();
}

gfx::RectShape ClientBlocks::getBlockRectInAtlas(BlockType* type) {
    return blocks_atlas.getRect(type->id - 1);
}

void ClientBlocks::render() {
    for(int x = getBlocksViewBeginX() / CHUNK_SIZE; x <= getBlocksViewEndX() / CHUNK_SIZE; x++)
        for(int y = getBlocksViewBeginY() / CHUNK_SIZE; y <= getBlocksViewEndY() / CHUNK_SIZE; y++) {
            if(!getRenderBlockChunk(x, y)->isCreated())
                getRenderBlockChunk(x, y)->create();
            
            if(getRenderBlockChunk(x, y)->has_update)
                getRenderBlockChunk(x, y)->update(this, x, y); 
            
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

int ClientBlocks::getBlocksViewBeginX() {
    return view_begin_x;
}

int ClientBlocks::getBlocksViewEndX() {
    return view_end_x;
}

int ClientBlocks::getBlocksViewBeginY() {
    return view_begin_y;
}

int ClientBlocks::getBlocksViewEndY() {
    return view_end_y;
}

int ClientBlocks::getBlocksExtendedViewBeginX() {
    return extended_view_begin_x;
}

int ClientBlocks::getBlocksExtendedViewEndX() {
    return extended_view_end_x;
}

int ClientBlocks::getBlocksExtendedViewBeginY() {
    return extended_view_begin_y;
}

int ClientBlocks::getBlocksExtendedViewEndY() {
    return extended_view_end_y;
}

