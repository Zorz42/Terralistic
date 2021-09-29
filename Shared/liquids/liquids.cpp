#include "liquids.hpp"
#include "graphics.hpp"

void Liquids::create() {
    liquids = new Liquid[blocks->getWidth() * blocks->getHeight()];
}

Liquids::Liquid* Liquids::getLiquid(unsigned short x, unsigned short y) {
    if(x >= blocks->getWidth() || y >= blocks->getHeight())
        throw LiquidOutOfBoundsException();
    return &liquids[y * blocks->getWidth() + x];
}

const LiquidInfo& Liquids::getLiquidInfo(unsigned short x, unsigned short y) {
    return ::getLiquidInfo(getLiquid(x, y)->type);
}

LiquidType Liquids::getLiquidType(unsigned short x, unsigned short y) {
    return getLiquid(x, y)->type;
}

void Liquids::setLiquidTypeSilently(unsigned short x, unsigned short y, LiquidType type) {
    if((int)type < 0 || type >= LiquidType::NUM_LIQUIDS)
        throw InvalidBlockTypeException();
    getLiquid(x, y)->type = type;
}

void Liquids::setLiquidType(unsigned short x, unsigned short y, LiquidType type) {
    if(type != getLiquidType(x, y)) {
        if(type == LiquidType::EMPTY)
            setLiquidLevel(x, y, 0);
        
        setLiquidTypeSilently(x, y, type);
        
        LiquidChangeEvent event(x, y);
        liquid_change_event.call(event);
    }
}

void Liquids::scheduleLiquidUpdate(unsigned short x, unsigned short y) {
    getLiquid(x, y)->when_to_update = gfx::getTicks() + getLiquidInfo(x, y).flow_time;
}

bool Liquids::canUpdateLiquid(unsigned short x, unsigned short y) {
    return getLiquid(x, y)->when_to_update && gfx::getTicks() > getLiquid(x, y)->when_to_update;
}

bool Liquids::isFlowable(unsigned short x, unsigned short y) {
    return blocks->getBlockInfo(x, y).ghost && getLiquidType(x, y) == LiquidType::EMPTY;
}

void Liquids::updateLiquid(unsigned short x, unsigned short y) {
    getLiquid(x, y)->when_to_update = 0;
    
    if(!blocks->getBlockInfo(x, y).ghost)
        setLiquidType(x, y, LiquidType::EMPTY);
    
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
        
        short liquid_sum = getLiquidLevel(x, y + 1) + getLiquidLevel(x, y);
        if(liquid_sum > 127) {
            setLiquidLevel(x, y + 1, 127);
            setLiquidLevel(x, y, liquid_sum - 127);
        } else {
            setLiquidType(x, y, LiquidType::EMPTY);
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
        short avg = (getLiquidLevel(x, y) + getLiquidLevel(x + 1, y) + getLiquidLevel(x - 1, y)) / 3;
        short mod = (getLiquidLevel(x, y) + getLiquidLevel(x + 1, y) + getLiquidLevel(x - 1, y)) % 3;
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
        short avg = (getLiquidLevel(x, y) + getLiquidLevel(x + 1, y)) / 2;
        short mod = (getLiquidLevel(x, y) + getLiquidLevel(x + 1, y)) % 2;
        setLiquidLevel(x + 1, y, avg + mod);
        getLiquid(x + 1, y)->flow_direction = FlowDirection::RIGHT;
        setLiquidLevel(x, y, avg);
        
    } else if(left_exists) {
        short avg = (getLiquidLevel(x, y) + getLiquidLevel(x - 1, y)) / 2;
        short mod = (getLiquidLevel(x, y) + getLiquidLevel(x - 1, y)) % 2;
        setLiquidLevel(x - 1, y, avg + mod);
        getLiquid(x - 1, y)->flow_direction = FlowDirection::LEFT;
        setLiquidLevel(x, y, avg);
    }
}

void Liquids::setLiquidLevelSilently(unsigned short x, unsigned short y, unsigned char level) {
    getLiquid(x, y)->level = level;
}

void Liquids::setLiquidLevel(unsigned short x, unsigned short y, unsigned char level) {
    if(level != getLiquidLevel(x, y)) {
        setLiquidLevelSilently(x, y, level);
        if(level == 0)
            setLiquidType(x, y, LiquidType::EMPTY);
        
        LiquidChangeEvent event(x, y);
        liquid_change_event.call(event);
    }
}

unsigned char Liquids::getLiquidLevel(unsigned short x, unsigned short y) {
    return getLiquid(x, y)->level;
}

void Liquids::serialize(std::vector<char>& serial) {
    serial.reserve(serial.size() + blocks->getWidth() * blocks->getHeight() * 2);
    Liquid* liquid = liquids;
    for(int y = 0; y < blocks->getHeight(); y++)
        for(int x = 0; x < blocks->getWidth(); x++) {
            if(blocks->getBlockInfo(x, y).ghost) {
                serial.push_back((char)liquid->type);
                if(getLiquidType(x, y) != LiquidType::EMPTY)
                    serial.push_back((char)liquid->level);
            }
            liquid++;
        }
}

char* Liquids::loadFromSerial(char* iter) {
    Liquid* liquid = liquids;
    for(int y = 0; y < blocks->getHeight(); y++)
        for(int x = 0; x < blocks->getWidth(); x++) {
            if(blocks->getBlockInfo(x, y).ghost) {
                liquid->type = (LiquidType)*iter++;
                if(liquid->type != LiquidType::EMPTY)
                    liquid->level = *iter++;
            }
            liquid++;
        }
    return iter;
}
