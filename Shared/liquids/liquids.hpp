#pragma once

#include "blocks.hpp"

enum class FlowDirection {NONE, LEFT, RIGHT};

class LiquidChangeEvent {
public:
    LiquidChangeEvent(int x, int y) : x(x), y(y) {}
    int x, y;
};

class LiquidType {
public:
    LiquidType() = default;
    LiquidType(std::string name, unsigned short flow_time, float speed_multiplier, gfx::Color color);
    
    std::string name;
    unsigned short flow_time;
    float speed_multiplier;
    gfx::Color color;
    unsigned char id;
};

namespace LiquidTypes {
    inline LiquidType empty(/*name*/"empty", /*flow_time*/0, /*speed_multiplier*/1, /*color*/{0, 0, 0, 0});
}

class Liquids {
    struct Liquid {
        LiquidTypeOld type:8;
        FlowDirection flow_direction:8;
        unsigned char level = 0;
        unsigned int when_to_update = 1;
    };
    
    std::vector<LiquidType*> liquid_types;
    
    Liquid* liquids = nullptr;
    Liquid* getLiquid(int x, int y);
    bool isFlowable(int x, int y);
    
    Blocks* blocks;
public:
    Liquids(Blocks* blocks) : blocks(blocks) { registerNewLiquidType(&LiquidTypes::empty); }
    
    void create();
    
    int getWidth() const;
    int getHeight() const;
    
    const LiquidInfoOld& getLiquidInfo(int x, int y);
    LiquidTypeOld getLiquidType(int x, int y);
    void setLiquidTypeSilently(int x, int y, LiquidTypeOld type);
    void setLiquidType(int x, int y, LiquidTypeOld type);
    
    void scheduleLiquidUpdate(int x, int y);
    bool canUpdateLiquid(int x, int y);
    void updateLiquid(int x, int y);
    
    unsigned char getLiquidLevel(int x, int y);
    void setLiquidLevel(int x, int y, unsigned char level);
    void setLiquidLevelSilently(int x, int y, unsigned char level);
    
    void serialize(std::vector<char>& serial);
    char* loadFromSerial(char* iter);
    
    void registerNewLiquidType(LiquidType* liquid_type);
    LiquidType* getLiquidTypeById(unsigned char liquid_id);
    unsigned char getNumLiquidTypes();
    
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
