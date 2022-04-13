#include "liquids.hpp"
#include "compress.hpp"

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

LiquidType* Liquids::getLiquidTypeByName(const std::string& name) {
    for(LiquidType* type : liquid_types)
        if(type->name == name)
            return type;
    return nullptr;
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

bool Liquids::isFlowable(int x, int y) {
    return blocks->getBlockType(x, y)->ghost && getLiquidType(x, y) == &empty;
}

void Liquids::updateLiquid(int x, int y) {
    if(getLiquidLevel(x, y) == 0) {
        setLiquidType(x, y, &empty);
        return;
    }
    
    if(!blocks->getBlockType(x, y)->ghost)
        setLiquidType(x, y, &empty);
    
    bool under_exists = false, left_exists = false, right_exists = false;
    
    if(y < getHeight() - 1 && (isFlowable(x, y + 1) || (getLiquidType(x, y + 1) == getLiquidType(x, y) && getLiquidLevel(x, y + 1) != MAX_LIQUID_LEVEL)))
        under_exists = true;
    
    if(x > 0 && (isFlowable(x - 1, y) || (getLiquidType(x - 1, y) == getLiquidType(x, y) && getLiquidLevel(x - 1, y) < getLiquidLevel(x, y))))
        left_exists = true;
    
    if(x < getWidth() - 1 && (isFlowable(x + 1, y) || (getLiquidType(x + 1, y) == getLiquidType(x, y) && getLiquidLevel(x + 1, y) < getLiquidLevel(x, y))))
        right_exists = true;
    
    
    if(under_exists) {
        setLiquidType(x, y + 1, getLiquidType(x, y));
        
        float liquid_sum = getLiquidLevel(x, y + 1) + getLiquidLevel(x, y);
        if(liquid_sum > MAX_LIQUID_LEVEL) {
            setLiquidLevel(x, y + 1, MAX_LIQUID_LEVEL);
            setLiquidLevel(x, y, liquid_sum - MAX_LIQUID_LEVEL);
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
    
    if(left_exists && right_exists && int(getLiquidLevel(x + 1, y)) != int(getLiquidLevel(x, y)) && int(getLiquidLevel(x - 1, y)) != int(getLiquidLevel(x, y))) {
        float avg = (getLiquidLevel(x, y) + getLiquidLevel(x + 1, y) + getLiquidLevel(x - 1, y)) / 3;

        setLiquidLevel(x - 1, y, avg);
        setLiquidLevel(x + 1, y, avg);
        setLiquidLevel(x, y, avg);
        
    } else if(right_exists && int(getLiquidLevel(x + 1, y)) != int(getLiquidLevel(x, y))) {
        float avg = (getLiquidLevel(x, y) + getLiquidLevel(x + 1, y)) / 2;
        setLiquidLevel(x + 1, y, avg);
        setLiquidLevel(x, y, avg);
        
    } else if(left_exists && int(getLiquidLevel(x - 1, y)) != int(getLiquidLevel(x, y))) {
        float avg = (getLiquidLevel(x, y) + getLiquidLevel(x - 1, y)) / 2;
        setLiquidLevel(x - 1, y, avg);
        setLiquidLevel(x, y, avg);
    }
}

void Liquids::setLiquidLevelSilently(int x, int y, float level) {
    getLiquid(x, y)->level = level;
}

void Liquids::setLiquidLevel(int x, int y, float level) {
    if(level != getLiquidLevel(x, y)) {
        setLiquidLevelSilently(x, y, level);
        if(level == 0)
            setLiquidType(x, y, &empty);
        
        LiquidChangeEvent event(x, y);
        liquid_change_event.call(event);
    }
}

float Liquids::getLiquidLevel(int x, int y) {
    return getLiquid(x, y)->level;
}

std::vector<char> Liquids::toSerial() {
    std::vector<char> serial;
    serial.reserve(blocks->getWidth() * blocks->getHeight() * 2);
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
    return compress(serial);
}

void Liquids::fromSerial(const std::vector<char>& serial) {
    std::vector<char> decompressed = decompress(serial);
    create();
    const char* iter = &decompressed[0];
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
