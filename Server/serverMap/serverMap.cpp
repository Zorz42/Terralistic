//
//  serverMap.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/04/2021.
//

#include "serverMap.hpp"
#include "assert.hpp"
#include "serverNetworking.hpp"

serverMap::~serverMap() {
    for(player* i : all_players)
        delete i;
}
