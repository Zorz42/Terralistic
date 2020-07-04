//
//  terrainGenerator.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#ifndef terrainGenerator_h
#define terrainGenerator_h

namespace terrainGenerator {

void generateTerrain(unsigned int seed);
void generatingScreen();
inline long loading_current = 0, loading_total = -1;

}

#endif /* terrainGenerator_h */
