#pragma once
#include <utility>

#include "blocks.hpp"

class WallChangeEvent {
public:
    WallChangeEvent(int x, int y) : x(x), y(y) {}
    int x, y;
};

class WallBreakEvent {
public:
    WallBreakEvent(int x, int y) : x(x), y(y) {}
    int x, y;
};

class WallStartedBreakingEvent {
public:
    WallStartedBreakingEvent(int x, int y) : x(x), y(y) {}
    int x, y;
};

class WallStoppedBreakingEvent {
public:
    WallStoppedBreakingEvent(int x, int y) : x(x), y(y) {}
    int x, y;
};

class WallType : public NonCopyable {
public:
    explicit WallType(std::string name) : name(std::move(name)) {}
    
    std::string name;
    int break_time = 0;
    int id = 0;
};

class Walls : public NonCopyable {
    class Wall {
    public:
        Wall() : id(/*clear*/0) {}
        int id:8;
    };
    
    class BreakingWall {
    public:
        int break_progress = 0;
        bool is_breaking = true;
        int x = 0, y = 0;
    };
    
    class WallChunk {
    public:
        int breaking_wall_count = 0;
    };
    
    std::vector<Wall> walls;
    std::vector<WallChunk> chunks;
    Blocks* blocks;
    
    std::vector<BreakingWall> breaking_walls;
    std::vector<WallType*> wall_types;
    Wall* getWall(int x, int y);
    WallChunk* getChunk(int x, int y);
    
    int curr_id = 0;
public:
    explicit Walls(Blocks* blocks) : blocks(blocks), clear("clear"), hammer("hammer") { registerNewWallType(&clear); blocks->registerNewToolType(&hammer); clear.break_time = UNBREAKABLE; }
    void create();

    WallType clear;
    Tool hammer;
    
    WallType* getWallType(int x, int y);
    void setWallType(int x, int y, WallType* type);
    void setWallTypeSilently(int x, int y, WallType* type);
    
    int getWidth() const;
    int getHeight() const;
    
    int getBreakProgress(int x, int y);
    int getBreakStage(int x, int y);
    void startBreakingWall(int x, int y);
    void stopBreakingWall(int x, int y);
    void updateBreakingWalls(int frame_length);
    int getChunkBreakingWallsCount(int x, int y);
    
    void breakWall(int x, int y);
    
    std::vector<char> toSerial();
    void fromSerial(const std::vector<char>& serial);
    
    void registerNewWallType(WallType* wall_type);
    WallType* getWallTypeById(int wall_id);
    WallType* getWallTypeByName(const std::string& name);
    int getNumWallTypes();
    
    EventSender<WallChangeEvent> wall_change_event;
    EventSender<WallBreakEvent> wall_break_event;
    EventSender<WallStartedBreakingEvent> wall_started_breaking_event;
    EventSender<WallStoppedBreakingEvent> wall_stopped_breaking_event;
};
