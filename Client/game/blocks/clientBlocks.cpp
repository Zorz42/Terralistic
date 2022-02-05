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
            
            setBlockType(x, y, getBlockTypeById(block_id));
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
    if(x < 0 || x >= getWidth() / RENDER_BLOCK_CHUNK_SIZE || y < 0 || y >= getHeight() / RENDER_BLOCK_CHUNK_SIZE)
        throw Exception("Chunk is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
    return &block_chunks[y * getWidth() / RENDER_BLOCK_CHUNK_SIZE + x];
}

bool ClientBlocks::getBlockUpdate(int x, int y) {
    if(x < 0 || x >= getWidth() || y < 0 || y >= getHeight())
        throw Exception("BlockUpdate is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
    return block_updates[y * getWidth() + x];
}

void ClientBlocks::setBlockUpdate(int x, int y, bool value) {
    if(x < 0 || x >= getWidth() || y < 0 || y >= getHeight())
        throw Exception("BlockUpdate is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
    block_updates[y * getWidth() + x] = value;
    if(value)
        getRenderBlockChunk(x / RENDER_BLOCK_CHUNK_SIZE, y / RENDER_BLOCK_CHUNK_SIZE)->has_update = true;
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
    block_updates = new bool[getWidth() * getHeight()];
    for(int i = 0; i < getWidth() * getHeight(); i++)
        block_updates[i] = false;
}

void ClientBlocks::onEvent(BlockChangeEvent& event) {
    setBlockUpdate(event.x, event.y, true);
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

void ClientBlocks::RenderBlockChunk::create(ClientBlocks* blocks, int x, int y) {
    block_rects.resize(RENDER_BLOCK_CHUNK_SIZE * RENDER_BLOCK_CHUNK_SIZE);
    is_created = true;
    
    for(int x2 = 0; x2 < RENDER_BLOCK_CHUNK_SIZE; x2++)
        for(int y2 = 0; y2 < RENDER_BLOCK_CHUNK_SIZE; y2++) {
            block_rects.setRect(RENDER_BLOCK_CHUNK_SIZE * y2 + x2, {x2 * BLOCK_WIDTH * 2, y2 * BLOCK_WIDTH * 2, BLOCK_WIDTH * 2, BLOCK_WIDTH * 2});
            update(blocks, x * RENDER_BLOCK_CHUNK_SIZE + x2, y * RENDER_BLOCK_CHUNK_SIZE + y2);
        }
}

void ClientBlocks::RenderBlockChunk::update(ClientBlocks* blocks, int x, int y) {
    if(!is_created)
        return;
    
    int rel_x = x % RENDER_BLOCK_CHUNK_SIZE, rel_y = y % RENDER_BLOCK_CHUNK_SIZE;
    int index = RENDER_BLOCK_CHUNK_SIZE * rel_y + rel_x;
    if(blocks->getBlockType(x, y) != &blocks->air) {
        if(blocks->getState(x, y) == 16)
            blocks->updateState(x, y);
        
        int texture_x = (blocks->getRenderBlock(x, y)->variation) % (blocks->getBlockRectInAtlas(blocks->getBlockType(x, y)).w / BLOCK_WIDTH) * BLOCK_WIDTH;
        int texture_y = blocks->getBlockRectInAtlas(blocks->getBlockType(x, y)).y + BLOCK_WIDTH * blocks->getRenderBlock(x, y)->state;
        block_rects.setTextureCoords(index, {texture_x, texture_y, BLOCK_WIDTH, BLOCK_WIDTH});
    } else
        block_rects.setTextureCoords(index, {0, 0, 0, 0});
}

void ClientBlocks::RenderBlockChunk::render(ClientBlocks* blocks, int x, int y) {
    block_rects.render(16 * 16, &blocks->getBlocksAtlasTexture(), x, y);
}

const gfx::Texture& ClientBlocks::getBlocksAtlasTexture() {
    return blocks_atlas.getTexture();
}

gfx::RectShape ClientBlocks::getBlockRectInAtlas(BlockType* type) {
    return blocks_atlas.getRect(type->id - 1);
}

void ClientBlocks::render() {
    for(int x = getBlocksViewBeginX() / RENDER_BLOCK_CHUNK_SIZE; x <= getBlocksViewEndX() / RENDER_BLOCK_CHUNK_SIZE; x++)
        for(int y = getBlocksViewBeginY() / RENDER_BLOCK_CHUNK_SIZE; y <= getBlocksViewEndY() / RENDER_BLOCK_CHUNK_SIZE; y++) {
            if(!getRenderBlockChunk(x, y)->isCreated())
                getRenderBlockChunk(x, y)->create(this, x, y);
            
            getRenderBlockChunk(x, y)->render(this, x * RENDER_BLOCK_CHUNK_SIZE * BLOCK_WIDTH * 2 - camera->getX() + gfx::getWindowWidth() / 2, y * RENDER_BLOCK_CHUNK_SIZE * BLOCK_WIDTH * 2 - camera->getY() + gfx::getWindowHeight() / 2);
            
            if(getChunkBreakingBlocksCount(x, y) > 0) {
                for(int x_ = x * RENDER_BLOCK_CHUNK_SIZE; x_ < (x + 1) * RENDER_BLOCK_CHUNK_SIZE; x_++)
                    for(int y_ = y * RENDER_BLOCK_CHUNK_SIZE; y_ < (y + 1) * RENDER_BLOCK_CHUNK_SIZE; y_++)
                        if(getBreakStage(x_, y_)) {
                            int block_x = x_ * BLOCK_WIDTH * 2 - camera->getX() + gfx::getWindowWidth() / 2, block_y = y_ * BLOCK_WIDTH * 2 - camera->getY() + gfx::getWindowHeight() / 2;
                            breaking_texture.render(2, block_x, block_y, gfx::RectShape(0, BLOCK_WIDTH * (getBreakStage(x_, y_) - 1), BLOCK_WIDTH, BLOCK_WIDTH));
                        }
            }
            
            if(getRenderBlockChunk(x, y)->has_update) {
                getRenderBlockChunk(x, y)->has_update = false;
                for(int x_ = x * RENDER_BLOCK_CHUNK_SIZE; x_ < (x + 1) * RENDER_BLOCK_CHUNK_SIZE; x_++)
                    for(int y_ = y * RENDER_BLOCK_CHUNK_SIZE; y_ < (y + 1) * RENDER_BLOCK_CHUNK_SIZE; y_++) {
                        setBlockUpdate(x_, y_, false);
                        int coords[5][2] = {{x_, y_}, {x_ + 1, y_}, {x_ - 1, y_}, {x_, y_ + 1}, {x_, y_ - 1}};
                        for(int i = 0; i < 5; i++) {
                            updateState(coords[i][0], coords[i][1]);
                            getRenderBlockChunk(coords[i][0] / 16, coords[i][1] / 16)->update(this, coords[i][0], coords[i][1]);
                        }
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

