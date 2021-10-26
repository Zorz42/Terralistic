#include "clientBlocks.hpp"

bool updateOrientationSide(ClientBlocks* blocks, int x, int y, char side_x, char side_y) {
    if(
            x + side_x >= blocks->getWidth() || x + side_x < 0 || y + side_y >= blocks->getHeight() || y + side_y < 0 ||
            blocks->getBlockType(x + side_x, y + side_y) == blocks->getBlockType(x, y) ||
            std::count(blocks->getBlockInfo(x, y).connects_to.begin(), blocks->getBlockInfo(x, y).connects_to.end(), blocks->getBlockType(x + side_x, y + side_y))
            )
        return true;
    else
        return false;
}

void updateOrientationDown(ClientBlocks* blocks, int x, int y) {
    blocks->setState(x, y, blocks->getState(x, y) * 2);
    if(updateOrientationSide(blocks, x, y, 0, 1))
        blocks->setState(x, y, blocks->getState(x, y) + 1);
}

void updateOrientationUp(ClientBlocks* blocks, int x, int y) {
    blocks->setState(x, y, blocks->getState(x, y) * 2);
    if(updateOrientationSide(blocks, x, y, 0, -1))
        blocks->setState(x, y, blocks->getState(x, y) + 1);
}

void updateOrientationLeft(ClientBlocks* blocks, int x, int y) {
    blocks->setState(x, y, blocks->getState(x, y) * 2);
    if(updateOrientationSide(blocks, x, y, -1, 0))
        blocks->setState(x, y, blocks->getState(x, y) + 1);
}

void updateOrientationRight(ClientBlocks* blocks, int x, int y) {
    blocks->setState(x, y, blocks->getState(x, y) * 2);
    if(updateOrientationSide(blocks, x, y, 1, 0))
        blocks->setState(x, y, blocks->getState(x, y) + 1);
}

ClientBlocks::ClientBlocks(ResourcePack* resource_pack, ClientNetworking* networking, Lights* lights) : resource_pack(resource_pack), networking(networking), lights(lights) {
    stateFunctions[(int)BlockType::DIRT] = std::vector<void (*)(ClientBlocks*, int, int)>{&updateOrientationLeft, &updateOrientationDown, &updateOrientationRight, &updateOrientationUp};
    stateFunctions[(int)BlockType::STONE_BLOCK] = std::vector<void (*)(ClientBlocks*, int, int)>{&updateOrientationLeft, &updateOrientationDown, &updateOrientationRight, &updateOrientationUp};
    stateFunctions[(int)BlockType::GRASS_BLOCK] = std::vector<void (*)(ClientBlocks*, int, int)>{&updateOrientationLeft, &updateOrientationDown, &updateOrientationRight, &updateOrientationUp};
    stateFunctions[(int)BlockType::WOOD] = std::vector<void (*)(ClientBlocks*, int, int)>{&updateOrientationLeft, &updateOrientationDown, &updateOrientationRight, &updateOrientationUp};
    stateFunctions[(int)BlockType::LEAVES] = std::vector<void (*)(ClientBlocks*, int, int)>{&updateOrientationLeft, &updateOrientationDown, &updateOrientationRight, &updateOrientationUp};
    stateFunctions[(int)BlockType::SAND] = std::vector<void (*)(ClientBlocks*, int, int)>{&updateOrientationLeft, &updateOrientationDown, &updateOrientationRight, &updateOrientationUp};
    stateFunctions[(int)BlockType::SNOWY_GRASS_BLOCK] = std::vector<void (*)(ClientBlocks*, int, int)>{&updateOrientationLeft, &updateOrientationDown, &updateOrientationRight, &updateOrientationUp};
    stateFunctions[(int)BlockType::SNOW_BLOCK] = std::vector<void (*)(ClientBlocks*, int, int)>{&updateOrientationLeft, &updateOrientationDown, &updateOrientationRight, &updateOrientationUp};
    stateFunctions[(int)BlockType::ICE] = std::vector<void (*)(ClientBlocks*, int, int)>{&updateOrientationLeft, &updateOrientationDown, &updateOrientationRight, &updateOrientationUp};
}

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
            unsigned char block_type;
            event.packet >> x >> y >> block_type;
            
            setBlockType(x, y, (BlockType)block_type);
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
    for(auto stateFunction : stateFunctions[(int)getBlockType(x, y)])
        stateFunction(this, x, y);
}

void ClientBlocks::setState(int x, int y, unsigned char state) {
    getRenderBlock(x, y)->state = state;
}

unsigned char ClientBlocks::getState(int x, int y) {
    return getRenderBlock(x, y)->state;
}

void ClientBlocks::postInit() {
    render_blocks = new RenderBlock[getWidth() * getHeight()];
    view_x = getWidth() * BLOCK_WIDTH;
    view_y = 0;
}

ClientBlocks::~ClientBlocks() {
    delete[] render_blocks;
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
}

void ClientBlocks::render() {
    if((getViewEndX() - getViewBeginX()) * (getViewEndY() - getViewBeginY()) > most_blocks_on_screen) {
        most_blocks_on_screen = (getViewEndX() - getViewBeginX()) * (getViewEndY() - getViewBeginY());
        block_rects.resize(most_blocks_on_screen);
    }
    
    int block_index = 0;
    for(unsigned short x = getViewBeginX(); x < getViewEndX(); x++)
        for(unsigned short y = getViewBeginY(); y < getViewEndY(); y++) {
            if(getState(x, y) == 16)
                updateState(x, y);
            
            if(getBlockType(x, y) != BlockType::AIR && lights->getLightLevel(x, y)) {
                int block_x = x * BLOCK_WIDTH * 2 - view_x + gfx::getWindowWidth() / 2, block_y = y * BLOCK_WIDTH * 2 - view_y + gfx::getWindowHeight() / 2;
                int texture_x = (getRenderBlock(x, y)->variation) % (resource_pack->getTextureRectangle(getBlockType(x, y)).w / BLOCK_WIDTH) * BLOCK_WIDTH;
                int texture_y = resource_pack->getTextureRectangle(getBlockType(x, y)).y + BLOCK_WIDTH * getRenderBlock(x, y)->state;
                
                block_rects.setTextureCoords(block_index, {(short)texture_x, (short)texture_y, BLOCK_WIDTH, BLOCK_WIDTH});
                block_rects.setRect(block_index, {(short)block_x, (short)block_y, BLOCK_WIDTH * 2, BLOCK_WIDTH * 2});
                
                block_index++;
            }
        }
    
    if(block_index)
        block_rects.render(block_index, &resource_pack->getBlockTexture());
    
    for(unsigned short x = getViewBeginX(); x < getViewEndX(); x++)
        for(unsigned short y = getViewBeginY(); y < getViewEndY(); y++) {
            if(getBreakStage(x, y)) {
                int block_x = x * BLOCK_WIDTH * 2 - view_x + gfx::getWindowWidth() / 2, block_y = y * BLOCK_WIDTH * 2 - view_y + gfx::getWindowHeight() / 2;
                resource_pack->getBreakingTexture().render(2, block_x, block_y, gfx::RectShape(0, short(BLOCK_WIDTH * (getBreakStage(x, y) - 1)), BLOCK_WIDTH, BLOCK_WIDTH));
            }
        }
}
