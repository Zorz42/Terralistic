#pragma once

#include "blocks.hpp"

enum class FlowDirection {NONE, LEFT, RIGHT};

class LiquidChangeEvent {
public:
    LiquidChangeEvent(int x, int y) : x(x), y(y) {}
    int x, y;
};

class Liquids {
    struct Liquid {
        LiquidType type:8;
        FlowDirection flow_direction:8;
        unsigned char level = 0;
        unsigned int when_to_update = 1;
    };
    
    Liquid* liquids = nullptr;
    Liquid* getLiquid(int x, int y);
    bool isFlowable(int x, int y);
    
    Blocks* blocks;
public:
    Liquids(Blocks* blocks) : blocks(blocks) {}
    
    void create();
    
    unsigned short getWidth();
    unsigned short getHeight();
    
    const LiquidInfo& getLiquidInfo(int x, int y);
    LiquidType getLiquidType(int x, int y);
    void setLiquidTypeSilently(int x, int y, LiquidType type);
    void setLiquidType(int x, int y, LiquidType type);
    
    void scheduleLiquidUpdate(int x, int y);
    bool canUpdateLiquid(int x, int y);
    void updateLiquid(int x, int y);
    
    unsigned char getLiquidLevel(int x, int y);
    void setLiquidLevel(int x, int y, unsigned char level);
    void setLiquidLevelSilently(int x, int y, unsigned char level);
    
    void serialize(std::vector<char>& serial);
    char* loadFromSerial(char* iter);
    
    EventSender<LiquidChangeEvent> liquid_change_event;
    
    ~Liquids();
};

class LiquidOutOfBoundsException : public std::exception {
public:
    const char* what() const throw() {
        return "Liquid is accessed out of the bounds!";
    }
};

class InvalidLiquidTypeException : public std::exception {
public:
    const char* what() const throw() {
        return "Liquid type does not exist!";
    }
};
