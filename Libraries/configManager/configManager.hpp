//
//  configManager.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 28/05/2021.
//

#ifndef configManager_hpp
#define configManager_hpp

#include <fstream>
#include <string>
#include <map>

class configFile {
    std::string path;
    
    std::map<std::string, std::string> values;
public:
    configFile(std::string path);
    configFile() = default;
    void save();
    ~configFile();
    
    std::string get(std::string key);
    void set(std::string key, std::string value);
    void setDefault(std::string key, std::string value);
};

#endif /* configManager_hpp */
