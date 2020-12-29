//
//  uniqueBlock.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/12/2020.
//

#include "blockEngine.hpp"
#include "singleWindowLibrary.hpp"

blockEngine::uniqueBlock::uniqueBlock(const std::string& name, bool ghost, bool only_on_floor, bool transparent, itemEngine::itemType drop) : name(name), ghost(ghost), only_on_floor(only_on_floor), transparent(transparent), drop(drop) {
    int h = 0;
    texture = name == "air" ? nullptr : swl::loadTextureFromFile("texturePack/blocks/" + name + ".png", nullptr, &h);
    single_texture = h == 8;
}
