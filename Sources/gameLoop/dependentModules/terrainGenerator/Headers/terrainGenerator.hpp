//
//  terrainGenerator.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#ifndef terrainGenerator_h
#define terrainGenerator_h

#include "perlinNoise.hpp"

namespace terrainGenerator {

int generateTerrainDaemon(unsigned int seed);
inline long loading_current = 0, loading_total = -1;
double turbulence(double x, double y, double size, double x_period, double y_period, double turb_power, PerlinNoise noise);

}

#endif /* terrainGenerator_h */
