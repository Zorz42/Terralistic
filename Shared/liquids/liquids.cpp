#include "liquids.hpp"

LiquidType::LiquidType(std::string name, int flow_time, float speed_multiplier, gfx::Color color) : name(std::move(name)), flow_time(flow_time), speed_multiplier(speed_multiplier), color(color) {}

void Liquids::create() {
    liquids = new Liquid[blocks->getWidth() * blocks->getHeight()];
}

Liquids::Liquid* Liquids::getLiquid(int x, int y) {
    if(x < 0 || x >= blocks->getWidth() || y < 0 || y >= blocks->getHeight())
        throw Exception("Liquid is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
    return &liquids[y * blocks->getWidth() + x];
}

LiquidType* Liquids::getLiquidType(int x, int y) {
    return getLiquidTypeById(getLiquid(x, y)->id);
}

void Liquids::setLiquidTypeSilently(int x, int y, LiquidType* type) {
    getLiquid(x, y)->id = type->id;
}

void Liquids::setLiquidType(int x, int y, LiquidType* type) {
    if(type != getLiquidType(x, y)) {
        if(type == &empty)
            setLiquidLevel(x, y, 0);
        
        setLiquidTypeSilently(x, y, type);
        
        LiquidChangeEvent event(x, y);
        liquid_change_event.call(event);
    }
}

void Liquids::scheduleLiquidUpdate(int x, int y) {
    getLiquid(x, y)->when_to_update = gfx::getTicks() + getLiquidType(x, y)->flow_time;
}

bool Liquids::canUpdateLiquid(int x, int y) {
    return getLiquid(x, y)->when_to_update && gfx::getTicks() > getLiquid(x, y)->when_to_update;
}

bool Liquids::isFlowable(int x, int y) {
    return blocks->getBlockType(x, y)->ghost && getLiquidType(x, y) == &empty;
}

void Liquids::updateLiquid(int x, int y) {
    getLiquid(x, y)->when_to_update = 0;
    
    if(!blocks->getBlockType(x, y)->ghost)
        setLiquidType(x, y, &empty);
    
    if(getLiquidLevel(x, y) == 0)
        return;
    
    bool under_exists = false, left_exists = false, right_exists = false;
    
    if(y < blocks->getHeight() && (isFlowable(x, y + 1) || (getLiquidType(x, y + 1) == getLiquidType(x, y) && getLiquidLevel(x, y + 1) != 127)))
        under_exists = true;
    
    if(x >= 0 && (isFlowable(x - 1, y) || (getLiquidType(x - 1, y) == getLiquidType(x, y) && getLiquidLevel(x - 1, y) < getLiquidLevel(x, y))))
        left_exists = true;
    
    if(x < blocks->getWidth() && (isFlowable(x + 1, y) || (getLiquidType(x + 1, y) == getLiquidType(x, y) && getLiquidLevel(x + 1, y) < getLiquidLevel(x, y))))
        right_exists = true;
    
    
    if(under_exists) {
        setLiquidType(x, y + 1, getLiquidType(x, y));
        
        int liquid_sum = getLiquidLevel(x, y + 1) + getLiquidLevel(x, y);
        if(liquid_sum > 127) {
            setLiquidLevel(x, y + 1, 127);
            setLiquidLevel(x, y, liquid_sum - 127);
        } else {
            setLiquidType(x, y, &empty);
            setLiquidLevel(x, y + 1, liquid_sum);
        }
        
        getLiquid(x, y + 1)->flow_direction = FlowDirection::NONE;
    }
    
    if(getLiquidLevel(x, y) == 0)
        return;
    
    if(right_exists)
        setLiquidType(x + 1, y, getLiquidType(x, y));
    if(left_exists)
        setLiquidType(x - 1, y, getLiquidType(x, y));
    
    if(left_exists && right_exists) {
        int avg = (getLiquidLevel(x, y) + getLiquidLevel(x + 1, y) + getLiquidLevel(x - 1, y)) / 3;
        int mod = (getLiquidLevel(x, y) + getLiquidLevel(x + 1, y) + getLiquidLevel(x - 1, y)) % 3;
        if(mod) {
            if(getLiquid(x, y)->flow_direction == FlowDirection::LEFT) {
                setLiquidLevel(x - 1, y, avg + mod);
                getLiquid(x - 1, y)->flow_direction = FlowDirection::LEFT;
                setLiquidLevel(x + 1, y, avg);
            } else {
                setLiquidLevel(x + 1, y, avg + mod);
                getLiquid(x + 1, y)->flow_direction = FlowDirection::RIGHT;
                setLiquidLevel(x - 1, y, avg);
            }
        } else {
            getLiquid(x - 1, y)->flow_direction = FlowDirection::NONE;
            setLiquidLevel(x - 1, y, avg);
            getLiquid(x + 1, y)->flow_direction = FlowDirection::NONE;
            setLiquidLevel(x + 1, y, avg);
        }
        
        setLiquidLevel(x, y, avg);
        getLiquid(x, y)->flow_direction = FlowDirection::NONE;
        
    } else if(right_exists) {
        int avg = (getLiquidLevel(x, y) + getLiquidLevel(x + 1, y)) / 2;
        int mod = (getLiquidLevel(x, y) + getLiquidLevel(x + 1, y)) % 2;
        setLiquidLevel(x + 1, y, avg + mod);
        getLiquid(x + 1, y)->flow_direction = FlowDirection::RIGHT;
        setLiquidLevel(x, y, avg);
        
    } else if(left_exists) {
        int avg = (getLiquidLevel(x, y) + getLiquidLevel(x - 1, y)) / 2;
        int mod = (getLiquidLevel(x, y) + getLiquidLevel(x - 1, y)) % 2;
        setLiquidLevel(x - 1, y, avg + mod);
        getLiquid(x - 1, y)->flow_direction = FlowDirection::LEFT;
        setLiquidLevel(x, y, avg);
    }
}

void Liquids::setLiquidLevelSilently(int x, int y, int level) {
    getLiquid(x, y)->level = level;
}

void Liquids::setLiquidLevel(int x, int y, int level) {
    if(level != getLiquidLevel(x, y)) {
        setLiquidLevelSilently(x, y, level);
        if(level == 0)
            setLiquidType(x, y, &empty);
        
        LiquidChangeEvent event(x, y);
        liquid_change_event.call(event);
    }
}

int Liquids::getLiquidLevel(int x, int y) {
    return getLiquid(x, y)->level;
}

void Liquids::serialize(std::vector<char>& serial) {
    serial.reserve(serial.size() + blocks->getWidth() * blocks->getHeight() * 2);
    Liquid* liquid = liquids;
    for(int y = 0; y < blocks->getHeight(); y++)
        for(int x = 0; x < blocks->getWidth(); x++) {
            if(blocks->getBlockType(x, y)->ghost) {
                serial.push_back((char)liquid->id);
                if(getLiquidType(x, y) != &empty)
                    serial.push_back((char)liquid->level);
            }
            liquid++;
        }
}

char* Liquids::loadFromSerial(char* iter) {
    create();
    Liquid* liquid = liquids;
    for(int y = 0; y < blocks->getHeight(); y++)
        for(int x = 0; x < blocks->getWidth(); x++) {
            if(blocks->getBlockType(x, y)->ghost) {
                liquid->id = *iter++;
                if(liquid->id != empty.id)
                    liquid->level = *iter++;
            }
            liquid++;
        }
    return iter;
}

int Liquids::getWidth() const {
    return blocks->getWidth();
}

int Liquids::getHeight() const {
    return blocks->getHeight();
}

void Liquids::registerNewLiquidType(LiquidType* liquid_type) {
    liquid_type->id = (int)liquid_types.size();
    liquid_types.push_back(liquid_type);
}

LiquidType* Liquids::getLiquidTypeById(int liquid_id) {
    if(liquid_id < 0 || liquid_id >= liquid_types.size())
        throw Exception("Liquid id is out of bounds.");
    return liquid_types[liquid_id];
}

int Liquids::getNumLiquidTypes() {
    return (int)liquid_types.size();
}

Liquids::~Liquids() {
    delete[] liquids;
}
