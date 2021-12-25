#pragma once
#include "clientBlocks.hpp"
#include "walls.hpp"

class ClientWalls : public Walls, public ClientModule {
public:
    ClientWalls(Blocks* blocks) : Walls(blocks) {}
};
