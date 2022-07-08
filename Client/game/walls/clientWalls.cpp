#include "clientWalls.hpp"
#include "readOpa.hpp"

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
    if(render_walls == nullptr)
        throw Exception("render_walls are null");
    return &render_walls[y * getWidth() + x];
}

ClientWalls::RenderWallChunk* ClientWalls::getRenderWallChunk(int x, int y) {
    if(wall_chunks == nullptr)
        throw Exception("wall_chunks are null");
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
    if(event.packet_type == WelcomePacketType::WALLS) {
        std::vector<char> data;
        event.packet >> data;
        fromSerial(data);
    }
}

void ClientWalls::updateParallel(float frame_length) {
    updateBreakingWalls(frame_length);
    for(int y = blocks->getBlocksViewBeginY() / CHUNK_SIZE; y <= blocks->getBlocksViewEndY() / CHUNK_SIZE; y++)
        for(int x = blocks->getBlocksViewBeginX() / CHUNK_SIZE; x <= blocks->getBlocksViewEndX() / CHUNK_SIZE; x++) {
            if(!getRenderWallChunk(x, y)->isCreated())
                getRenderWallChunk(x, y)->create();
            
            if(getRenderWallChunk(x, y)->has_update)
                getRenderWallChunk(x, y)->update(this, x, y);
        }
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
    for(auto & coord : coords) {
        scheduleWallUpdate(coord[0], coord[1]);
        updateState(coord[0], coord[1]);
    }
}

void ClientWalls::scheduleWallUpdate(int x, int y) {
    getRenderWallChunk(x / CHUNK_SIZE, y / CHUNK_SIZE)->has_update = true;
}

void ClientWalls::init() {
    networking->welcome_packet_event.addListener(this);
    networking->packet_event.addListener(this);
    wall_change_event.addListener(this);
    debug_menu->registerDebugLine(&render_time_line);
}

void ClientWalls::loadTextures() {
    loadOpa(breaking_texture, resource_pack->getFile("/misc/breaking.opa"));
    
    std::vector<gfx::Texture*> wall_textures(getNumWallTypes() - 1);

    for(int i = 1; i < getNumWallTypes(); i++) {
        wall_textures[i - 1] = new gfx::Texture;
        loadOpa(*wall_textures[i - 1], resource_pack->getFile("/walls/" + getWallTypeById(i)->name + ".opa"));
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

void ClientWalls::RenderWallChunk::update(ClientWalls* walls, int x, int y) {
    has_update = false;
    int index = 0;
    for(int y_ = y * CHUNK_SIZE; y_ < (y + 1) * CHUNK_SIZE; y_++)
        for(int x_ = x * CHUNK_SIZE; x_ < (x + 1) * CHUNK_SIZE; x_++)
            if(walls->getWallType(x_, y_) != &walls->clear) {
                if(walls->getState(x_, y_) == 16)
                    walls->updateState(x_, y_);
                wall_rects.setRect(index, {(x_ % CHUNK_SIZE) * BLOCK_WIDTH * 2 - BLOCK_WIDTH * 2, (y_ % CHUNK_SIZE) * BLOCK_WIDTH * 2 - BLOCK_WIDTH * 2, BLOCK_WIDTH * 6, BLOCK_WIDTH * 6});
                
                int texture_x = (walls->getRenderWall(x_, y_)->variation) % (walls->getWallRectInAtlas(walls->getWallType(x_, y_)).w / BLOCK_WIDTH / 3) * 3 * BLOCK_WIDTH;
                int texture_y = walls->getWallRectInAtlas(walls->getWallType(x_, y_)).y + BLOCK_WIDTH * 3 * walls->getRenderWall(x_, y_)->state;
                wall_rects.setTextureCoords(index, {texture_x, texture_y, BLOCK_WIDTH * 3, BLOCK_WIDTH * 3});
                wall_rects.setColor(index, {255, 255, 255});
                index++;
            }
    wall_count = index;
}

void ClientWalls::RenderWallChunk::render(ClientWalls* walls, int x, int y) {
    if(wall_count > 0)
        wall_rects.render(&walls->getWallsAtlasTexture(), x, y, false, wall_count);
}

void ClientWalls::RenderWallChunk::create() {
    wall_rects.resize(CHUNK_SIZE * CHUNK_SIZE);
    is_created = true;
}

const gfx::Texture& ClientWalls::getWallsAtlasTexture() {
    return walls_atlas.getTexture();
}

gfx::RectShape ClientWalls::getWallRectInAtlas(WallType* type) {
    return walls_atlas.getRect(type->id - 1);
}

void ClientWalls::render() {
    gfx::Timer render_timer;
    for(int y = blocks->getBlocksViewBeginY() / CHUNK_SIZE; y <= blocks->getBlocksViewEndY() / CHUNK_SIZE; y++)
        for(int x = blocks->getBlocksViewBeginX() / CHUNK_SIZE; x <= blocks->getBlocksViewEndX() / CHUNK_SIZE; x++)
            if(getRenderWallChunk(x, y)->isCreated()) {
                getRenderWallChunk(x, y)->render(this, x * CHUNK_SIZE * BLOCK_WIDTH * 2 - camera->getX() + gfx::getWindowWidth() / 2, y * CHUNK_SIZE * BLOCK_WIDTH * 2 - camera->getY() + gfx::getWindowHeight() / 2);
                
                if(getChunkBreakingWallsCount(x, y) > 0) {
                    for(int x_ = x * CHUNK_SIZE; x_ < (x + 1) * CHUNK_SIZE; x_++)
                        for(int y_ = y * CHUNK_SIZE; y_ < (y + 1) * CHUNK_SIZE; y_++)
                            if(getBreakStage(x_, y_)) {
                                int block_x = x_ * BLOCK_WIDTH * 2 - camera->getX() + gfx::getWindowWidth() / 2, block_y = y_ * BLOCK_WIDTH * 2 - camera->getY() + gfx::getWindowHeight() / 2;
                                breaking_texture.render(2, block_x, block_y, gfx::RectShape(0, BLOCK_WIDTH * (getBreakStage(x_, y_) - 1), BLOCK_WIDTH, BLOCK_WIDTH));
                            }
                }
            }
    
    render_time_sum += render_timer.getTimeElapsed();
    fps_count++;
    if(line_refresh_timer.getTimeElapsed() >= 1000) {
        render_time_line.text = std::to_string(render_time_sum / fps_count) + "ms walls render";
        
        fps_count = 0;
        render_time_sum = 0;
        line_refresh_timer.reset();
    }
}

