#pragma once
#include "blocks.hpp"

class WallType {
public:
    WallType(std::string name) : name(name) {}
    
    std::string name;
    int id;
};

class Walls {
    class Wall {
    public:
        Wall() : id(/*clear*/0) {}
        int id:8;
    };
    
    Wall* walls = nullptr;
    Blocks* blocks;
    
    std::vector<WallType*> wall_types;
    Wall* getWall(int x, int y);
    
    int curr_id = 0;
public:
    Walls(Blocks* blocks) : blocks(blocks), clear("clear") { registerNewWallType(&clear); }
    void create();

    WallType clear;
    
    WallType* getWallType(int x, int y);
    void setWallTypeSilently(int x, int y, WallType* type);
    
    int getWidth() const;
    int getHeight() const;
    
    std::vector<char> toSerial();
    void fromSerial(const std::vector<char>& serial);
    
    void registerNewWallType(WallType* wall_type);
    WallType* getWallTypeById(int wall_id);
    WallType* getWallTypeByName(const std::string& name);
    int getNumWallTypes();
};
