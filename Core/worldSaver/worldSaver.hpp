//
//  worldSaver.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/12/2020.
//

#ifndef worldSaver_hpp
#define worldSaver_hpp

#include "map.hpp"

namespace worldSaver {

void saveWorld(const std::string& world_path, map& world_map);
void loadWorld(const std::string& world_path, map& world_map);
bool worldExists(const std::string& world_path);

}

#endif /* worldSaver_hpp */
