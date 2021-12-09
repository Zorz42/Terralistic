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
    LiquidType(std::string name) : name(std::move(name)) {}
    
    std::string name;
    int flow_time;
    float speed_multiplier;
    gfx::Color color;
    int id;
};

class Liquids {
    class Liquid {
    public:
        Liquid() : id(/*empty*/0), level(0) {}
        int id:8;
        FlowDirection flow_direction:8;
        int level:8;
        int when_to_update = 1;
    };
    
    std::vector<LiquidType*> liquid_types;
    Liquid* liquids = nullptr;
    Liquid* getLiquid(int x, int y);
    bool isFlowable(int x, int y);
    
    Blocks* blocks;
public:
    Liquids(Blocks* blocks) : blocks(blocks), empty("empty") { empty.flow_time = 0; empty.speed_multiplier = 1; empty.color = TRANSPARENT; registerNewLiquidType(&empty); }
    
    void create();
    
    LiquidType empty;
    
    int getWidth() const;
    int getHeight() const;
    
    LiquidType* getLiquidType(int x, int y);
    void setLiquidTypeSilently(int x, int y, LiquidType* type);
    void setLiquidType(int x, int y, LiquidType* type);
    
    void scheduleLiquidUpdate(int x, int y);
    bool canUpdateLiquid(int x, int y);
    void updateLiquid(int x, int y);
    
    int getLiquidLevel(int x, int y);
    void setLiquidLevel(int x, int y, int level);
    void setLiquidLevelSilently(int x, int y, int level);
    
    void serialize(std::vector<char>& serial);
    char* loadFromSerial(char* iter);
    
    void registerNewLiquidType(LiquidType* liquid_type);
    LiquidType* getLiquidTypeById(int liquid_id);
    int getNumLiquidTypes();
    
    EventSender<LiquidChangeEvent> liquid_change_event;
    
    ~Liquids();
};
