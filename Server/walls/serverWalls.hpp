#pragma once
#include "serverBlocks.hpp"
#include "walls.hpp"

class ServerWalls : public ServerModule, public Walls {
public:
    ServerWalls(Blocks* blocks) : Walls(blocks) {}
};
