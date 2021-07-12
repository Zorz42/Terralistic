#ifndef fileManager_hpp
#define fileManager_hpp

#include <string>

namespace fileManager {

void init();

std::string getResourcePath(std::string executable_path);

std::string getDataPath();
std::string getWorldsPath();

}

#endif /* fileManager_hpp */
