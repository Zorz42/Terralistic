#ifndef liquids_hpp
#define liquids_hpp

#include "blocks.hpp"
#include "properties.hpp"
#include "events.hpp"

enum class FlowDirection {NONE, LEFT, RIGHT};

class LiquidChangeEvent {
public:
    LiquidChangeEvent(unsigned short x, unsigned short y) : x(x), y(y) {}
    unsigned short x, y;
};

class Liquids {
    struct Liquid {
        LiquidType type:8;
        FlowDirection flow_direction:8;
        unsigned char level = 0;
        unsigned int when_to_update = 1;
    };
    
    Liquid* liquids = nullptr;
    unsigned short width, height;
    Liquid* getLiquid(unsigned short x, unsigned short y);
    bool isFlowable(unsigned short x, unsigned short y);
    
    Blocks* blocks;
public:
    Liquids(Blocks* blocks) : blocks(blocks) {}
    
    void create(unsigned short width, unsigned short height);
    
    const LiquidInfo& getLiquidInfo(unsigned short x, unsigned short y);
    LiquidType getLiquidType(unsigned short x, unsigned short y);
    void setLiquidTypeSilently(unsigned short x, unsigned short y, LiquidType type);
    void setLiquidType(unsigned short x, unsigned short y, LiquidType type);
    
    void scheduleLiquidUpdate(unsigned short x, unsigned short y);
    bool canUpdateLiquid(unsigned short x, unsigned short y);
    void updateLiquid(unsigned short x, unsigned short y);
    
    unsigned char getLiquidLevel(unsigned short x, unsigned short y);
    void setLiquidLevel(unsigned short x, unsigned short y, unsigned char level);
    void setLiquidLevelSilently(unsigned short x, unsigned short y, unsigned char level);
    
    EventSender<LiquidChangeEvent> liquid_change_event;
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

#endif
