#ifndef lights_hpp
#define lights_hpp

#include "blocks.hpp"

class LightSourceChangeEvent {
public:
    LightSourceChangeEvent(unsigned short x, unsigned short y) : x(x), y(y) {}
    unsigned short x, y;
};

class Lights {
    struct Light {
        bool light_source = false, update_light = true;
        unsigned char light_level = 0;
    };
    
    Light* lights;
    
    Light* getLight(unsigned short x, unsigned short y);
    void setLightLevel(unsigned short x, unsigned short y, unsigned char level);
    
    Blocks* blocks;
public:
    Lights(Blocks* blocks) : blocks(blocks) {}
    void create();
    
    void updateLight(unsigned short x, unsigned short y);
    void setLightSource(unsigned short x, unsigned short y, unsigned char level);
    unsigned char getLightLevel(unsigned short x, unsigned short y);
    void scheduleLightUpdate(unsigned short x, unsigned short y);
    bool hadScheduledLightUpdate(unsigned short x, unsigned short y);
};

class LightOutOfBoundsException : public std::exception {
public:
    const char* what() const throw() {
        return "Light is accessed out of the bounds!";
    }
};

#endif
