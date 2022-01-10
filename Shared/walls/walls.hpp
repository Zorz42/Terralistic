#pragma once
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

class WallType {
public:
    WallType(std::string name) : name(name) {}
    
    std::string name;
    int break_time;
    int id;
};

class Walls {
    class Wall {
    public:
        Wall() : id(/*clear*/0) {}
        int id:8;
    };
    
    class BreakingWall {
    public:
        int break_progress = 0;
        bool is_breaking = true;
        int x, y;
    };
    
    Wall* walls = nullptr;
    Blocks* blocks;
    
    std::vector<BreakingWall> breaking_walls;
    std::vector<WallType*> wall_types;
    Wall* getWall(int x, int y);
    
    int curr_id = 0;
public:
    Walls(Blocks* blocks) : blocks(blocks), clear("clear"), hammer("hammer") { registerNewWallType(&clear); blocks->registerNewToolType(&hammer); clear.break_time = UNBREAKABLE; }
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
