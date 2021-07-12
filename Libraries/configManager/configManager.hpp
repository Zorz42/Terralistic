#ifndef configManager_hpp
#define configManager_hpp

#include <string>
#include <map>

class ConfigFile {
    std::string path;
    std::map<std::string, std::string> values;
    
    void loadConfig();
    void saveConfig();
public:
    explicit ConfigFile(const std::string& path);
    ConfigFile() = default;
    ~ConfigFile();
    
    std::string getStr(const std::string& key);
    int getInt(const std::string& key);
    void setStr(const std::string& key, std::string value);
    void setInt(const std::string& key, int value);

    void setDefaultStr(const std::string& key, std::string value);
    void setDefaultInt(const std::string& key, int value);
    
    bool keyExists(const std::string& key);
};

#endif /* configManager_hpp */
