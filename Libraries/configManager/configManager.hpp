#pragma once

#include <map>
#include <string>

class ConfigFile {
    std::string path;
    std::map<std::string, std::string> values;
    
    void loadConfig();
public:
    explicit ConfigFile(std::string path);
    ConfigFile() = default;
    ~ConfigFile();
    
    std::string getStr(const std::string& key);
    int getInt(const std::string& key);
    void setStr(const std::string& key, std::string value);
    void setInt(const std::string& key, int value);

    void setDefaultStr(const std::string& key, std::string value);
    void setDefaultInt(const std::string& key, int value);
    
    bool keyExists(const std::string& key) const;
    
    void saveConfig();
};
