#include "blockEngine.hpp"
#include "caveGenerator.hpp"

caveGenerator::caveGenerator(int width, int height, unsigned int seed) : height(height), width(width), noise(seed) {
    map = new int[height*width];
    for(int i = 0; i < height * width; i++)
        map[i] = 0;
}

caveGenerator::~caveGenerator() {
    delete map;
}

void caveGenerator::evolveMap(RuleSet rule) {
    // By initializing column in the outer loop, its only created ONCE
    for (int row = 0; row <= height - 1; row++)
        for (int column = 0; column <= width - 1; column++)
            map[getIndex(column, row)] = placeWallLogic(column, row, rule);
}

int caveGenerator::placeWallLogic(int x, int y, RuleSet rule) {
    int numWalls = getAdjacentWalls(x, y, 1, 1);
    int numWalls2 = getAdjacentWalls(x, y, 2, 2);

    if (map[getIndex(x,y)] == 1)
        return (numWalls >= 3) ? 1 : 0;
    else {
        if (rule == caveGenerator::CM_CONSERVATIVE) {
            if (numWalls >= 5 || numWalls2 <= 2)
                return 1;
            else if (numWalls >= 5)
                    return 1;
        }
    }
    return 0;
}

int caveGenerator::getAdjacentWalls(int x, int y, int scope_x, int scope_y) {
    int startX = x - scope_x;
    int startY = y - scope_y;
    int endX = x + scope_x;
    int endY = y + scope_y;

    int wallCounter = 0;

    for (int iY = startY; iY <= endY; iY++)
        for (int iX = startX; iX <= endX; iX++)
            if (!(iX == x && iY == y))
                if (isWall(iX,iY))
                    wallCounter++;
    return wallCounter;
}

bool caveGenerator::isOutOfBound(int x, int y) {
    return (x < 0 || y<0 || x>width - 1 || y > height - 1);
}

bool caveGenerator::isWall(int x, int y) {
    // Out of bound is considered a wall.
    return (isOutOfBound(x, y)) || (map[getIndex(x, y)] == 1);
}

void caveGenerator::generateMap(double cave_start, double cave_length, int cave_cap, double x_period, double y_period, double turb_power, double turb_size, unsigned int highest_height) {
    // generate perlin noise caves into a cellular map
    for(unsigned int y = cave_cap; y < highest_height; y++)
        for(unsigned int x = 0; x < blockEngine::world_width; x++)
            getElement(x, y - cave_cap) = caveGenerator::turbulence(x, y - cave_cap, turb_size, x_period, y_period, turb_power, highest_height) > cave_start + cave_length - (double)y / highest_height * cave_length;
}

double caveGenerator::turbulence(double x, double y, double size, double x_period, double y_period, double turb_power, unsigned int highest_height) {
    double value = 0.0, initial_size = size;
    while(size >= 1) {
        value += noise.noise(x / size, y / size, 0.8) * size;
        size /= 2.0;
    }
    
    double xy_value = x * x_period / blockEngine::world_width + y * y_period / highest_height + turb_power * (value / initial_size) / 2.0;
    return fabs(sin(xy_value * 3.14159));
}
