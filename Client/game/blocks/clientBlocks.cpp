#include "clientBlocks.hpp"

bool ClientBlocks::updateOrientationSide(ClientBlocks* blocks, int x, int y, int side_x, int side_y) {
    if(
            x + side_x >= blocks->getWidth() || x + side_x < 0 || y + side_y >= blocks->getHeight() || y + side_y < 0 ||
            blocks->getBlockType(x + side_x, y + side_y) == blocks->getBlockType(x, y) ||
            std::count(blocks->getBlockType(x, y)->connects_to.begin(), blocks->getBlockType(x, y)->connects_to.end(), blocks->getBlockType(x + side_x, y + side_y))
            )
        return true;
    else
        return false;
}

void ClientBlocks::updateOrientationDown(ClientBlocks* blocks, int x, int y) {
    blocks->setState(x, y, blocks->getState(x, y) * 2);
    if(updateOrientationSide(blocks, x, y, 0, 1))
        blocks->setState(x, y, blocks->getState(x, y) + 1);
}

void ClientBlocks::updateOrientationUp(ClientBlocks* blocks, int x, int y) {
    blocks->setState(x, y, blocks->getState(x, y) * 2);
    if(updateOrientationSide(blocks, x, y, 0, -1))
        blocks->setState(x, y, blocks->getState(x, y) + 1);
}

void ClientBlocks::updateOrientationLeft(ClientBlocks* blocks, int x, int y) {
    blocks->setState(x, y, blocks->getState(x, y) * 2);
    if(updateOrientationSide(blocks, x, y, -1, 0))
        blocks->setState(x, y, blocks->getState(x, y) + 1);
}

void ClientBlocks::updateOrientationRight(ClientBlocks* blocks, int x, int y) {
    blocks->setState(x, y, blocks->getState(x, y) * 2);
    if(updateOrientationSide(blocks, x, y, 1, 0))
        blocks->setState(x, y, blocks->getState(x, y) + 1);
}


ClientBlocks::ClientBlocks(ResourcePack* resource_pack, ClientNetworking* networking, Lights* lights) : resource_pack(resource_pack), networking(networking), lights(lights) {}

int ClientBlocks::getViewBeginX() const {
    return std::max(view_x / (BLOCK_WIDTH * 2) - gfx::getWindowWidth() / 2 / (BLOCK_WIDTH * 2) - 2, 0);
}

int ClientBlocks::getViewEndX() const {
    return std::min(view_x / (BLOCK_WIDTH * 2) + gfx::getWindowWidth() / 2 / (BLOCK_WIDTH * 2) + 2, (int)getWidth());
}

int ClientBlocks::getViewBeginY() const {
    return std::max(view_y / (BLOCK_WIDTH * 2) - gfx::getWindowHeight() / 2 / (BLOCK_WIDTH * 2) - 2, 0);
}

int ClientBlocks::getViewEndY() const {
    return std::min(view_y / (BLOCK_WIDTH * 2) + gfx::getWindowHeight() / 2 / (BLOCK_WIDTH * 2) + 2, (int)getHeight());
}

void ClientBlocks::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case ServerPacketType::BLOCK: {
            int x, y;
            int block_id;
            event.packet >> x >> y >> block_id;
            
            setBlockType(x, y, getBlockTypeById(block_id));
            break;
        }
        case ServerPacketType::STARTED_BREAKING: {
            int x, y;
            event.packet >> x >> y;
            startBreakingBlock(x, y);
            break;
        }
        case ServerPacketType::STOPPED_BREAKING: {
            int x, y;
            event.packet >> x >> y;
            stopBreakingBlock(x, y);
            break;
        }
        default:;
    }
}

ClientBlocks::RenderBlock* ClientBlocks::getRenderBlock(int x, int y) {
    return &render_blocks[y * getWidth() + x];
}

ClientBlocks::RenderBlockChunk* ClientBlocks::getRenderBlockChunk(int x, int y) {
    return &render_chunks[y * getWidth() / 16 + x];
}

void ClientBlocks::updateState(int x, int y) {
    getRenderBlock(x, y)->state = 0;
    if(resource_pack->getTextureRectangle(getBlockType(x, y)).h != 8) {
        updateOrientationLeft(this, x, y);
        updateOrientationDown(this, x, y);
        updateOrientationRight(this, x, y);
        updateOrientationUp(this, x, y);
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
    render_chunks = new RenderBlockChunk[getWidth() / 16 * getHeight() / 16];
    view_x = getWidth() * BLOCK_WIDTH;
    view_y = 0;
    
    for(int x = 0; x < getWidth(); x++)
        for(int y = 0; y < getHeight(); y++)
            getRenderBlockChunk(x / 16, y / 16)->update(this, resource_pack, x, y);
}

void ClientBlocks::onEvent(BlockChangeEvent& event) {
    updateState(event.x, event.y);
    updateState(event.x + 1, event.y);
    updateState(event.x - 1, event.y);
    updateState(event.x, event.y + 1);
    updateState(event.x, event.y - 1);
    getRenderBlockChunk(event.x / 16, event.y / 16)->update(this, resource_pack, event.x, event.y);
}

void ClientBlocks::onEvent(WelcomePacketEvent& event) {
    if(event.packet_type == WelcomePacketType::BLOCKS) {
        std::vector<char> data = networking->getData();
        loadFromSerial(&data[0]);
    }
}

void ClientBlocks::init() {
    block_change_event.addListener(this);
    networking->packet_event.addListener(this);
    networking->welcome_packet_event.addListener(this);
}

void ClientBlocks::update(float frame_length) {
    updateBreakingBlocks(frame_length);
}

void ClientBlocks::stop() {
    block_change_event.removeListener(this);
    networking->packet_event.removeListener(this);
    networking->welcome_packet_event.removeListener(this);
    
    delete[] render_blocks;
}

void ClientBlocks::RenderBlockChunk::update(ClientBlocks* blocks, ResourcePack* resource_pack_, int x, int y) {
    int rel_x = x % BLOCK_CHUNK_SIZE, rel_y = y % BLOCK_CHUNK_SIZE;
    int index = BLOCK_CHUNK_SIZE * rel_y + rel_x;
    if(blocks->getBlockType(x, y) != &blocks->air) {
        if(blocks->getState(x, y) == 16)
            blocks->updateState(x, y);
        
        int texture_x = (blocks->getRenderBlock(x, y)->variation) % (resource_pack_->getTextureRectangle(blocks->getBlockType(x, y)).w / BLOCK_WIDTH) * BLOCK_WIDTH;
        int texture_y = resource_pack_->getTextureRectangle(blocks->getBlockType(x, y)).y + BLOCK_WIDTH * blocks->getRenderBlock(x, y)->state;
        block_rects.setTextureCoords(index, {texture_x, texture_y, BLOCK_WIDTH, BLOCK_WIDTH});
    } else {
        block_rects.setTextureCoords(index, {0, 0, 0, 0});
    }
    block_rects.setRect(index, {rel_x * BLOCK_WIDTH * 2, rel_y * BLOCK_WIDTH * 2, BLOCK_WIDTH * 2, BLOCK_WIDTH * 2});
}

void ClientBlocks::RenderBlockChunk::render(ResourcePack* resource_pack_, int x, int y) {
    block_rects.render(16 * 16, &resource_pack_->getBlockTexture(), x, y);
}

void ClientBlocks::render() {
    for(int x = getViewBeginX() / 16; x <= getViewEndX() / 16; x++)
        for(int y = getViewBeginY() / 16; y <= getViewEndY() / 16; y++)
            getRenderBlockChunk(x, y)->render(resource_pack, x * BLOCK_CHUNK_SIZE * BLOCK_WIDTH * 2 - view_x + gfx::getWindowWidth() / 2, y * BLOCK_CHUNK_SIZE * BLOCK_WIDTH * 2 - view_y + gfx::getWindowHeight() / 2);
    
    for(int x = getViewBeginX(); x < getViewEndX(); x++)
        for(int y = getViewBeginY(); y < getViewEndY(); y++) {
            if(getBreakStage(x, y)) {
                int block_x = x * BLOCK_WIDTH * 2 - view_x + gfx::getWindowWidth() / 2, block_y = y * BLOCK_WIDTH * 2 - view_y + gfx::getWindowHeight() / 2;
                resource_pack->getBreakingTexture().render(2, block_x, block_y, gfx::RectShape(0, BLOCK_WIDTH * (getBreakStage(x, y) - 1), BLOCK_WIDTH, BLOCK_WIDTH));
            }
        }
}
