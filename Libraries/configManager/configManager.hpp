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
    void loadConfig();


    explicit configFile(const std::string& path);
    configFile() = default;
    void save();
    ~configFile();
    
    std::string getStr(const std::string& key);
    int getInt(const std::string& key);
    void setStr(const std::string& key, std::string value);
    void setInt(const std::string& key, int value);

    void setDefaultStr(const std::string& key, std::string value);
    void setDefaultInt(const std::string& key, int value);
};

#endif /* configManager_hpp */