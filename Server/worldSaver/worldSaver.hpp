//
//  worldSaver.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/12/2020.
//

#ifndef worldSaver_hpp
#define worldSaver_hpp

#include "serverMap.hpp"

namespace worldSaver {

void saveWorld(const std::string& world_path, serverMap& world_serverMap);
void loadWorld(const std::string& world_path, serverMap& world_serverMap);
bool worldExists(const std::string& world_path);

}

#endif /* worldSaver_hpp */
