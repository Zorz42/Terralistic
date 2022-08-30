#pragma once
#include "blocks.hpp"

#define MAX_LIGHT 100

class LightColorChangeEvent {
public:
    LightColorChangeEvent(int x, int y) : x(x), y(y) {}
    int x, y;
};

class LightUpdateScheduleEvent {
public:
    LightUpdateScheduleEvent(int x, int y) : x(x), y(y) {}
    int x, y;
};

class LightColor {
public:
    LightColor(int r, int g, int b) : r(r), g(g), b(b) {}
    int r:8, g:8, b:8;
    bool operator==(LightColor color) const;
    bool operator!=(LightColor color) const;
};

class Lights : EventListener<BlockChangeEvent> {
    class Light {
    public:
        Light() : color(0, 0, 0), source_color(0, 0, 0) {}
        bool light_source = false, update_light = true;
        LightColor source_color;
        LightColor color;
    };
    
    std::vector<Light> lights;
    
    Light* getLight(int x, int y);
    void setLightColor(int x, int y, LightColor color);
    
    void onEvent(BlockChangeEvent& event) override;
    
    Blocks* blocks;
public:
    explicit Lights(Blocks* blocks) : blocks(blocks) {}
    void create();
    
    void init();
    void stop();
    
    int getWidth() const;
    int getHeight() const;
    
    void updateLight(int x, int y);
    void setLightSource(int x, int y, LightColor color);
    bool isLightSource(int x, int y);
    LightColor getLightColor(int x, int y);
    LightColor getLightSourceColor(int x, int y);
    void scheduleLightUpdate(int x, int y);
    bool hasScheduledLightUpdate(int x, int y);
    void scheduleLightUpdateForNeighbors(int x, int y);
    
    void updateLightEmitter(int x, int y);
    
    EventSender<LightColorChangeEvent> light_color_change_event;
    EventSender<LightUpdateScheduleEvent> light_update_schedule_event;
};
