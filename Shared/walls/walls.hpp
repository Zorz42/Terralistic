#pragma once
#include "blocks.hpp"

class WallType {
public:
    WallType(std::string name);
    
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
    
    std::vector<WallType*> wall_types;
    Wall* getWall(int x, int y);
public:
    Walls();
    void create();

    BlockType clear;
    
    BlockType* getWallType(int x, int y);
    void setWallTypeSilently(int x, int y, WallType* type);
    
    int getWidth() const;
    int getHeight() const;
    
    std::vector<char> serialize();
    void loadFromSerial(const std::vector<char>& serial);
    
    void registerNewWallType(WallType* wall_type);
    WallType* getWallTypeById(int wall_id);
    WallType* getWallTypeByName(const std::string& name);
    int getNumWallTypes();
};
