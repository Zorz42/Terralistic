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

/*!
 * Implementation of a Cellular Automata cave-like map generator.
 *
 * USAGE EXAMPLE:
 *      CellularMap cm(50,40,40);
 *      cm.evolveMap();
 */
class caveGenerator
{
public:

    /*!
     * Specifies the rule set used by the cellular automata.
     */
    enum RuleSet {
        CM_CONSERVATIVE,    //! Corresponding to R1(p) >= 5 || R2(p) <= 1
        CM_SMOOTH           //! Corresponding to R1(p) >= 5
    };

    /*!
     * Constructor.
     *
     * @param width                 Desired map width.
     * @param height                Desired map height.
     */
    caveGenerator(int width, int height, unsigned int seed);
    ~caveGenerator();

    /*!
     * Execute a step of the automata algorithm creating the cave or
     * Smoothing the existing ones using a specific rule set.
     */
    void evolveMap(RuleSet rule);
    
    // generate a rough map
    
    void generateMap(double cave_start, double cave_length, int cave_cap, double x_period, double y_period, double turb_power, double turb_size, unsigned int highest_height);
    
    /*!
     * Get the element in position <x,y>.
     *
     * @param x The x coordinate.
     * @param y The y coordinate.
     * @return The element in <x,y>.
     */
    inline int& getElement(int x, int y) { return map[getIndex(x, y)]; }
private:
    PerlinNoise noise;
    
    int* map;
    int width;              // The map width.
    int height;             // The map height.
    
    /*
    * Execute the Cellular Automata rules on every tile of the map.
    *
    * @return The new value of tile <x,y>.
    */
    int  placeWallLogic(int x, int y, RuleSet rule);

    /*
    * Get the sum of adjacent walls to a given tile <x,y>
    *
    * @param x          The x coordinate in the map.
    * @param y          The y coordinate in the map.
    * @param scope_x    The search scope radius in x direction.
    * @param scope_y    The search scope radius in y direction.
    * @return           The number of walls in the scope of <x,y>.
    */
    int  getAdjacentWalls(int x, int y, int scope_x, int scope_y);

    /*
    * Check if a tile is out of the map.
    *
    * @param x  The x coordinate in the map.
    * @param y  The y coordinate in the map.
    * @return   True iff <x,y> is out of the map. False otherwise.
    */
    bool isOutOfBound(int x, int y);

    /*
    * Get the rom-major index of the tile <x,y>
    *
    * @param x  The x coordinate in the map.
    * @param y  The y coordinate in the map.
    * @return   The row-major index of <x,y>.
    */
    int  inline getIndex(int x, int y) { return x + y*width; }

    /*
    * Chek if the tile is a Wall.
    *
    * @param x  The x coordinate in the map.
    * @param y  The y coordinate in the map.
    * @return   True iff <x,y> is a wall. False otherwise.
    */
    bool isWall(int x, int y);
    
    double turbulence(double x, double y, double size, double x_period, double y_period, double turb_power, unsigned int highest_height);
};
