//
//  lightingEngine.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 28/12/2020.
//

#ifndef lightingEngine_hpp
#define lightingEngine_hpp

#define MAX_LIGHT 100

namespace lightingEngine {

struct lightBlock {
    unsigned char level = 0;
    bool source = false;
    void render(short x, short y) const;
    void update(bool update=true);
    unsigned short getX();
    unsigned short getY();
};

void init();
void prepare();
void close();

inline lightBlock* light_map;

void removeNaturalLight(unsigned short x);
void setNaturalLight(unsigned short x);

lightBlock& getLightBlock(unsigned short x, unsigned short y);

void addLightSource(unsigned short x, unsigned short y, unsigned char power);
void removeLightSource(unsigned short x, unsigned short y);

}

#endif /* lightingEngine_hpp */
