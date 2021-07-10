//
//  configManager.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 28/05/2021.
//

#ifndef configManager_hpp
#define configManager_hpp

#include <string>
#include <map>

class configFile {
    std::string path;
    std::map<std::string, std::string> values;
    
    void loadConfig();
    void saveConfig();
public:
    explicit configFile(const std::string& path);
    configFile() = default;
    ~configFile();
    
    std::string getStr(const std::string& key);
    int getInt(const std::string& key);
    void setStr(const std::string& key, std::string value);
    void setInt(const std::string& key, int value);

    void setDefaultStr(const std::string& key, std::string value);
    void setDefaultInt(const std::string& key, int value);
    
    bool keyExists(const std::string& key);
};

#endif /* configManager_hpp */
