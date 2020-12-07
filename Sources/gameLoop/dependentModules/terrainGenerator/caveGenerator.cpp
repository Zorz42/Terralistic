#include "blockEngine.hpp"
#include "caveGenerator.hpp"
#include "terrainGenerator.hpp"

caveGenerator::caveGenerator(int width, int height, unsigned int seed) : height(height), width(width), noise(seed) {
    map = new int[height*width];
    for(int i = 0; i < height * width; i++)
        map[i] = 0;
}

caveGenerator::~caveGenerator() {
    delete map;
}

void caveGenerator::evolveMap(RuleSet rule) {
    for (int row = 0; row <= height - 1; row++)
        for (int column = 0; column <= width - 1; column++) {
            int numWalls = getAdjacentWalls(column, row, 1);
            map[getIndex(column, row)] = map[getIndex(column, row)] == 1 ? numWalls >= 3 : rule == caveGenerator::CM_CONSERVATIVE && (numWalls >= 5 || getAdjacentWalls(column, row, 2) <= 2);
        }
}

int caveGenerator::getAdjacentWalls(int x, int y, int scope) {
    int wallCounter = 0;

    for (int iY = y - scope; iY <= y + scope; iY++)
        for (int iX = x - scope; iX <= x + scope; iX++)
            if(!(iX == x && iY == y) && (iX < 0 || iY < 0 || iX > width - 1 || iY > height - 1 || map[getIndex(iX, iY)] == 1))
                wallCounter++;
    return wallCounter;
}

void caveGenerator::generateMap(double cave_start, double cave_length, int cave_cap, double x_period, double y_period, double turb_power, double turb_size, unsigned int highest_height) {
    // generate perlin noise caves into a cellular map
    for(unsigned int y = cave_cap; y < highest_height; y++)
        for(unsigned int x = 0; x < blockEngine::world_width; x++)
            getElement(x, y - cave_cap) = terrainGenerator::turbulence(x, y - cave_cap, turb_size, x_period, y_period, turb_power, highest_height, noise) > cave_start + cave_length - (double)y / highest_height * cave_length;
}
