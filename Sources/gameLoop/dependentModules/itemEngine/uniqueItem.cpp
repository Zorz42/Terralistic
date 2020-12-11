//
//  uniqueItem.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 10/12/2020.
//

#include "itemEngine.hpp"
#include "singleWindowLibrary.hpp"

itemEngine::uniqueItem::uniqueItem(std::string name) : name(name) {
    texture = name == "nothing" ? nullptr : swl::loadTextureFromFile("texturePack/items/" + name + ".png");
}
