#include "clientWalls.hpp"

bool ClientWalls::updateOrientationSide(int x, int y, int side_x, int side_y) {
    return x + side_x >= getWidth() || x + side_x < 0 || y + side_y >= getHeight() || y + side_y < 0 || getWallType(x + side_x, y + side_y) != &clear;
}

void ClientWalls::updateOrientationDown(int x, int y) {
    setState(x, y, getState(x, y) * 2);
    if(updateOrientationSide(x, y, 0, 1))
        setState(x, y, getState(x, y) + 1);
}

void ClientWalls::updateOrientationUp(int x, int y) {
    setState(x, y, getState(x, y) * 2);
    if(updateOrientationSide(x, y, 0, -1))
        setState(x, y, getState(x, y) + 1);
}

void ClientWalls::updateOrientationLeft(int x, int y) {
    setState(x, y, getState(x, y) * 2);
    if(updateOrientationSide(x, y, -1, 0))
        setState(x, y, getState(x, y) + 1);
}

void ClientWalls::updateOrientationRight(int x, int y) {
    setState(x, y, getState(x, y) * 2);
    if(updateOrientationSide(x, y, 1, 0))
        setState(x, y, getState(x, y) + 1);
}

ClientWalls::RenderWall* ClientWalls::getRenderWall(int x, int y) {
    return &render_walls[y * getWidth() + x];
}

ClientWalls::RenderWallChunk* ClientWalls::getWallChunk(int x, int y) {
    return &wall_chunks[y * getWidth() / 16 + x];
}

void ClientWalls::updateState(int x, int y) {
    getRenderWall(x, y)->state = 0;
    if(getWallType(x, y) != &clear && getWallRectInAtlas(getWallType(x, y)).h != 8) {
        updateOrientationLeft(x, y);
        updateOrientationDown(x, y);
        updateOrientationRight(x, y);
        updateOrientationUp(x, y);
    }
}

void ClientWalls::setState(int x, int y, int state) {
    getRenderWall(x, y)->state = state;
}

int ClientWalls::getState(int x, int y) {
    return getRenderWall(x, y)->state;
}

void ClientWalls::postInit() {
    render_walls = new RenderWall[getWidth() * getHeight()];
    wall_chunks = new RenderWallChunk[getWidth() / 16 * getHeight() / 16];
}

void ClientWalls::onEvent(WelcomePacketEvent& event) {
    if(event.packet_type == WelcomePacketType::WALLS)
        fromSerial(event.data);
}

void ClientWalls::update(float frame_length) {
    updateBreakingWalls(frame_length);
}

void ClientWalls::onEvent(ClientPacketEvent& event) {
    switch(event.packet_type) {
        case ServerPacketType::WALL: {
            int x, y;
            int wall_id;
            event.packet >> x >> y >> wall_id;
            
            setWallType(x, y, getWallTypeById(wall_id));
            break;
        }
        case ServerPacketType::WALL_STARTED_BREAKING: {
            int x, y;
            event.packet >> x >> y;
            startBreakingWall(x, y);
            break;
        }
        case ServerPacketType::WALL_STOPPED_BREAKING: {
            int x, y;
            event.packet >> x >> y;
            stopBreakingWall(x, y);
            break;
        }
        default:;
    }
}

void ClientWalls::onEvent(WallChangeEvent& event) {
    int coords[5][2] = {{event.x, event.y}, {event.x + 1, event.y}, {event.x - 1, event.y}, {event.x, event.y + 1}, {event.x, event.y - 1}};
    for(int i = 0; i < 5; i++) {
        updateState(coords[i][0], coords[i][1]);
        getWallChunk(coords[i][0] / 16, coords[i][1] / 16)->update(this, coords[i][0], coords[i][1]);
    }
}

void ClientWalls::init() {
    networking->welcome_packet_event.addListener(this);
    networking->packet_event.addListener(this);
    wall_change_event.addListener(this);
}

void ClientWalls::loadTextures() {
    breaking_texture.loadFromFile(resource_pack->getFile("/misc/breaking.png"));
    
    std::vector<gfx::Texture*> wall_textures(getNumWallTypes() - 1);

    for(int i = 1; i < getNumWallTypes(); i++) {
        wall_textures[i - 1] = new gfx::Texture;
        wall_textures[i - 1]->loadFromFile(resource_pack->getFile("/walls/" + getWallTypeById(i)->name + ".png"));
    }
    
    walls_atlas.create(wall_textures);
    
    for(int i = 1; i < getNumWallTypes(); i++)
        delete wall_textures[i - 1];
}

void ClientWalls::stop() {
    networking->welcome_packet_event.removeListener(this);
    networking->packet_event.removeListener(this);
    wall_change_event.removeListener(this);
    
    delete[] render_walls;
    delete[] wall_chunks;
}

void ClientWalls::RenderWallChunk::create(ClientWalls* walls, int x, int y) {
    wall_rects.resize(RENDER_WALL_CHUNK_SIZE * RENDER_WALL_CHUNK_SIZE);
    is_created = true;
    
    for(int x2 = 0; x2 < RENDER_WALL_CHUNK_SIZE; x2++)
        for(int y2 = 0; y2 < RENDER_WALL_CHUNK_SIZE; y2++) {
            wall_rects.setRect(RENDER_WALL_CHUNK_SIZE * y2 + x2, {x2 * BLOCK_WIDTH * 2 - BLOCK_WIDTH * 2, y2 * BLOCK_WIDTH * 2 - BLOCK_WIDTH * 2, BLOCK_WIDTH * 6, BLOCK_WIDTH * 6});
            update(walls, x * RENDER_WALL_CHUNK_SIZE + x2, y * RENDER_WALL_CHUNK_SIZE + y2);
        }
}

void ClientWalls::RenderWallChunk::update(ClientWalls* walls, int x, int y) {
    if(!is_created)
        return;
    
    int rel_x = x % RENDER_WALL_CHUNK_SIZE, rel_y = y % RENDER_WALL_CHUNK_SIZE;
    int index = RENDER_WALL_CHUNK_SIZE * rel_y + rel_x;
    if(walls->getWallType(x, y) != &walls->clear) {
        if(walls->getState(x, y) == 16)
            walls->updateState(x, y);
        
        int texture_x = (walls->getRenderWall(x, y)->variation) % (walls->getWallRectInAtlas(walls->getWallType(x, y)).w / BLOCK_WIDTH / 3) * 3 * BLOCK_WIDTH;
        int texture_y = walls->getWallRectInAtlas(walls->getWallType(x, y)).y + BLOCK_WIDTH * 3 * walls->getRenderWall(x, y)->state;
        wall_rects.setTextureCoords(index, {texture_x, texture_y, BLOCK_WIDTH * 3, BLOCK_WIDTH * 3});
    } else {
        wall_rects.setTextureCoords(index, {0, 0, 0, 0});
    }
}

void ClientWalls::RenderWallChunk::render(ClientWalls* walls, int x, int y) {
    wall_rects.render(16 * 16, &walls->getWallsAtlasTexture(), x, y);
}

const gfx::Texture& ClientWalls::getWallsAtlasTexture() {
    return walls_atlas.getTexture();
}

gfx::RectShape ClientWalls::getWallRectInAtlas(WallType* type) {
    return walls_atlas.getRect(type->id - 1);
}

void ClientWalls::render() {
    for(int x = blocks->getBlocksViewBeginX() / RENDER_WALL_CHUNK_SIZE; x <= blocks->getBlocksViewEndX() / RENDER_WALL_CHUNK_SIZE; x++)
        for(int y = blocks->getBlocksViewBeginY() / RENDER_WALL_CHUNK_SIZE; y <= blocks->getBlocksViewEndY() / RENDER_WALL_CHUNK_SIZE; y++) {
            if(!getWallChunk(x, y)->isCreated())
                getWallChunk(x, y)->create(this, x, y);
            
            getWallChunk(x, y)->render(this, x * RENDER_WALL_CHUNK_SIZE * BLOCK_WIDTH * 2 - camera->getX() + gfx::getWindowWidth() / 2, y * RENDER_WALL_CHUNK_SIZE * BLOCK_WIDTH * 2 - camera->getY() + gfx::getWindowHeight() / 2);
            
            if(getChunkBreakingWallsCount(x, y) > 0) {
                for(int x_ = x * RENDER_BLOCK_CHUNK_SIZE; x_ < (x + 1) * RENDER_BLOCK_CHUNK_SIZE; x_++)
                    for(int y_ = y * RENDER_BLOCK_CHUNK_SIZE; y_ < (y + 1) * RENDER_BLOCK_CHUNK_SIZE; y_++)
                        if(getBreakStage(x, y)) {
                            int block_x = x_ * BLOCK_WIDTH * 2 - camera->getX() + gfx::getWindowWidth() / 2, block_y = y_ * BLOCK_WIDTH * 2 - camera->getY() + gfx::getWindowHeight() / 2;
                            breaking_texture.render(2, block_x, block_y, gfx::RectShape(0, BLOCK_WIDTH * (getBreakStage(x_, y_) - 1), BLOCK_WIDTH, BLOCK_WIDTH));
                        }
            }
        }
}

