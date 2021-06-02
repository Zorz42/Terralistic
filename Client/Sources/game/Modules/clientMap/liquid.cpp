//
//  liquid.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 02/06/2021.
//

#include "clientMap.hpp"

std::vector<map::uniqueLiquid> map::unique_liquids;

map::uniqueLiquid::uniqueLiquid(const std::string& name) {
    texture.setTexture(gfx::loadImageFile("texturePack/liquids/" + name + ".png"));
    texture.scale = 2;
    texture.free_texture = false;
}

void map::initLiquids() {
    unique_liquids = {
        {"empty"},
        {"water"},
    };
}
