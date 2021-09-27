#ifndef dayCycle_hpp
#define dayCycle_hpp

#include "serverBlocks.hpp"
#include "events.hpp"

class DayCycle : EventListener<ServerBlockChangeEvent> {
    ServerBlocks* blocks;
    unsigned char *lights, *lights_should_be;
    unsigned int started;
    
    void onEvent(ServerBlockChangeEvent& event);
public:
    DayCycle(ServerBlocks* blocks) : blocks(blocks) {}
    void init();
    void update();
    
    void setNaturalLight(unsigned short x, unsigned char power);
};

#endif
