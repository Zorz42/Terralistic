#pragma once

#include "clientBlocks.hpp"

class Particles : public ClientModule {
    Blocks* blocks;
    
    void render() override;
public:
    Particles(Blocks* blocks) : blocks(blocks) {}
};
