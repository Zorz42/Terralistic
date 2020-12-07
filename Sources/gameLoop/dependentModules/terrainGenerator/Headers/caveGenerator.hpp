//
//  cellularMap.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 27/06/2020.
//

#pragma once

#include "perlinNoise.hpp"

// modified by me to remake it into a more complex perlin noise cave generator

/*!
* A simple but effective Cellular Automata 2D Map implementation.
*
* @author   Davide Aversa <thek3nger@gmail.com>
* @version  1.0
*/

class caveGenerator
{
public:
    enum RuleSet {
        CM_CONSERVATIVE,
        CM_SMOOTH
    };

    caveGenerator(int width, int height, unsigned int seed);
    ~caveGenerator();

    void evolveMap(RuleSet rule);
    
    void generateMap(double cave_start, double cave_length, int cave_cap, double x_period, double y_period, double turb_power, double turb_size, unsigned int highest_height);
    
    inline int& getElement(int x, int y) { return map[getIndex(x, y)]; }
private:
    PerlinNoise noise;
    
    int* map;
    int width;
    int height;

    int getAdjacentWalls(int x, int y, int scope);
    int inline getIndex(int x, int y) { return x + y*width; }
};
