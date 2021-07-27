#ifndef fileManager_hpp
#define fileManager_hpp

#include <string>

namespace fileManager {

void init();

std::string getDataPath();
std::string getWorldsPath();
std::string getConfigPath();

}

#endif
