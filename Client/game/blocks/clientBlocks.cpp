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
    view_x = getWidth() * BLOCK_WIDTH;
    view_y = 0;
}

void ClientBlocks::onEvent(BlockChangeEvent& event) {
    updateState(event.x, event.y);
    updateState(event.x + 1, event.y);
    updateState(event.x - 1, event.y);
    updateState(event.x, event.y + 1);
    updateState(event.x, event.y - 1);
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

void ClientBlocks::render() {
    if((getViewEndX() - getViewBeginX()) * (getViewEndY() - getViewBeginY()) > most_blocks_on_screen) {
        most_blocks_on_screen = (getViewEndX() - getViewBeginX()) * (getViewEndY() - getViewBeginY());
        block_rects.resize(most_blocks_on_screen);
    }
    
    int block_index = 0;
    for(int x = getViewBeginX(); x < getViewEndX(); x++)
        for(int y = getViewBeginY(); y < getViewEndY(); y++) {
            if(getState(x, y) == 16)
                updateState(x, y);
            
            if(getBlockType(x, y) != &BlockTypes_::air && (lights->getLightLevel(x, y) || !skip_rendering_in_dark)) {
                int block_x = x * BLOCK_WIDTH * 2 - view_x + gfx::getWindowWidth() / 2, block_y = y * BLOCK_WIDTH * 2 - view_y + gfx::getWindowHeight() / 2;
                int texture_x = (getRenderBlock(x, y)->variation) % (resource_pack->getTextureRectangle(getBlockType(x, y)).w / BLOCK_WIDTH) * BLOCK_WIDTH;
                int texture_y = resource_pack->getTextureRectangle(getBlockType(x, y)).y + BLOCK_WIDTH * getRenderBlock(x, y)->state;
                
                block_rects.setTextureCoords(block_index, {texture_x, texture_y, BLOCK_WIDTH, BLOCK_WIDTH});
                block_rects.setRect(block_index, {block_x, block_y, BLOCK_WIDTH * 2, BLOCK_WIDTH * 2});
                
                block_index++;
            }
        }
    
    if(block_index)
        block_rects.render(block_index, &resource_pack->getBlockTexture());
    
    for(int x = getViewBeginX(); x < getViewEndX(); x++)
        for(int y = getViewBeginY(); y < getViewEndY(); y++) {
            if(getBreakStage(x, y)) {
                int block_x = x * BLOCK_WIDTH * 2 - view_x + gfx::getWindowWidth() / 2, block_y = y * BLOCK_WIDTH * 2 - view_y + gfx::getWindowHeight() / 2;
                resource_pack->getBreakingTexture().render(2, block_x, block_y, gfx::RectShape(0, BLOCK_WIDTH * (getBreakStage(x, y) - 1), BLOCK_WIDTH, BLOCK_WIDTH));
            }
        }
}
