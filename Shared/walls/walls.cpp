#include "walls.hpp"
#include "compress.hpp"

Walls::Wall* Walls::getWall(int x, int y) {
    if(x < 0 || x >= getWidth() || y < 0 || y >= getHeight())
        throw Exception("Wall is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
    return &walls[y * getWidth() + x];
}

void Walls::create() {
    walls = new Wall[getWidth() * getHeight()];
}

WallType* Walls::getWallType(int x, int y) {
    return getWallTypeById(getWall(x, y)->id);
}

void Walls::setWallTypeSilently(int x, int y, WallType* type) {
    getWall(x, y)->id = type->id;
}

void Walls::setWallType(int x, int y, WallType* type) {
    if(type->id != getWall(x, y)->id) {
        setWallTypeSilently(x, y, type);
        
        WallChangeEvent event(x, y);
        wall_change_event.call(event);
    }
}

int Walls::getWidth() const {
    return blocks->getWidth();
}

int Walls::getHeight() const {
    return blocks->getHeight();
}

std::vector<char> Walls::toSerial() {
    std::vector<char> serial;
    unsigned long iter = 0;
    serial.resize(serial.size() + getWidth() * getHeight());
    Wall* wall = walls;
    for(int i = 0; i < getWidth() * getHeight(); i++) {
        serial[iter++] = (char)wall->id;
        wall++;
    }
    return compress(serial);
}

void Walls::fromSerial(const std::vector<char>& serial) {
    std::vector<char> decompressed = decompress(serial);
    const char* iter = &decompressed[0];
    create();
    Wall* wall = walls;
    for(int i = 0; i < getWidth() * getHeight(); i++) {
        wall->id = *iter++;
        wall++;
    }
}

void Walls::registerNewWallType(WallType* wall_type) {
    wall_type->id = curr_id++;
    wall_types.push_back(wall_type);
}

WallType* Walls::getWallTypeById(int wall_id) {
    return wall_types[wall_id];
}

WallType* Walls::getWallTypeByName(const std::string& name) {
    for(int i = 0; i < wall_types.size(); i++)
        if(wall_types[i]->name == name)
            return wall_types[i];
    return nullptr;
}

int Walls::getNumWallTypes() {
    return (int)wall_types.size();
}

int Walls::getBreakProgress(int x, int y) {
    for(int i = 0; i < breaking_walls.size(); i++)
        if(breaking_walls[i].x == x && breaking_walls[i].y == y)
            return breaking_walls[i].break_progress;
    return 0;
}

void Walls::updateBreakingWalls(int frame_length) {
    for(int i = 0; i < breaking_walls.size(); i++) {
        if(breaking_walls[i].is_breaking) {
            breaking_walls[i].break_progress += frame_length;
            if(breaking_walls[i].break_progress > getWallType(breaking_walls[i].x, breaking_walls[i].y)->break_time)
                breakWall(breaking_walls[i].x, breaking_walls[i].y);
        }
    }
}

int Walls::getBreakStage(int x, int y) {
    return (float)getBreakProgress(x, y) / (float)getWallType(x, y)->break_time * 9.f;
}

void Walls::startBreakingWall(int x, int y) {
    if(x < 0 || x >= getWidth() || y < 0 || y >= getHeight())
        throw Exception("Wall is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
    
    BreakingWall* breaking_wall = nullptr;
    
    for(int i = 0; i < breaking_walls.size(); i++)
        if(breaking_walls[i].x == x && breaking_walls[i].y == y)
            breaking_wall = &breaking_walls[i];
    
    if(!breaking_wall) {
        BreakingWall new_breaking_wall;
        new_breaking_wall.x = x;
        new_breaking_wall.y = y;
        breaking_walls.push_back(new_breaking_wall);
        breaking_wall = &breaking_walls.back();
    }
    
    breaking_wall->is_breaking = true;
        
    WallStartedBreakingEvent event(x, y);
    wall_started_breaking_event.call(event);
}

void Walls::stopBreakingWall(int x, int y) {
    if(x < 0 || x >= getWidth() || y < 0 || y >= getHeight())
        throw Exception("Wall is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
    
    for(int i = 0; i < breaking_walls.size(); i++)
        if(breaking_walls[i].x == x && breaking_walls[i].y == y) {
            breaking_walls[i].is_breaking = false;
            WallStoppedBreakingEvent event(x, y);
            wall_stopped_breaking_event.call(event);
        }
}

void Walls::breakWall(int x, int y) {
    WallBreakEvent event(x, y);
    wall_break_event.call(event);
    
    setWallType(x, y, &clear);
}
