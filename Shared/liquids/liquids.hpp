#pragma once
#include "blocks.hpp"

#define MAX_LIQUID_LEVEL 100

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
    int id;
};

class Liquids {
    class Liquid {
    public:
        Liquid() : id(/*empty*/0), level(0) {}
        int id:8;
        float level;
    };
    
    std::vector<LiquidType*> liquid_types;
    Liquid* liquids = nullptr;
    Liquid* getLiquid(int x, int y);
    bool isFlowable(int x, int y);
    
    Blocks* blocks;
public:
    explicit Liquids(Blocks* blocks) : blocks(blocks), empty("empty") { empty.flow_time = 0; empty.speed_multiplier = 1; registerNewLiquidType(&empty); }
    
    void create();
    
    LiquidType empty;
    
    int getWidth() const;
    int getHeight() const;
    
    LiquidType* getLiquidType(int x, int y);
    LiquidType* getLiquidTypeByName(const std::string& name);
    void setLiquidTypeSilently(int x, int y, LiquidType* type);
    void setLiquidType(int x, int y, LiquidType* type);
    
    void updateLiquid(int x, int y);
    
    float getLiquidLevel(int x, int y);
    void setLiquidLevel(int x, int y, float level);
    void setLiquidLevelSilently(int x, int y, float level);
    
    std::vector<char> toSerial();
    void fromSerial(const std::vector<char>& serial);
    
    void registerNewLiquidType(LiquidType* liquid_type);
    LiquidType* getLiquidTypeById(int liquid_id);
    int getNumLiquidTypes();
    
    EventSender<LiquidChangeEvent> liquid_change_event;
    
    ~Liquids();
};
