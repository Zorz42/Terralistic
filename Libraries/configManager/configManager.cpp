#include <utility>
#include <fstream>
#include "configManager.hpp"
#include "exception.hpp"

ConfigFile::ConfigFile(std::string path) : path(std::move(path)) {
    reloadFromDisk();
}

void ConfigFile::reloadFromDisk() {
    std::ifstream file(path);
    std::string line;
    while(std::getline(file, line)) {
        unsigned long separator_index = line.find(':');
        if(separator_index < line.length()) {
            std::string value = line.substr(separator_index + 1, line.size());
            while(!value.empty() && value[0] == ' ')
                value.erase(value.begin());
            values[line.substr(0, separator_index)] = value;
        }
    }
    file.close();
}

std::string ConfigFile::getStr(const std::string& key) {
    if(!keyExists(key))
        throw ConfigKeyError("Key \"" + key + "\" does not exist in config!");
    return values[key];
}

int ConfigFile::getInt(const std::string& key) {
    return std::stoi(getStr(key));
}

void ConfigFile::setStr(const std::string& key, std::string value) {
    values[key] = std::move(value);
}
void ConfigFile::setInt(const std::string& key, int value) {
    setStr(key, std::to_string(value));
}

void ConfigFile::setDefaultStr(const std::string& key, std::string value) {
    if(!keyExists(key))
        setStr(key, std::move(value));
}

void ConfigFile::setDefaultInt(const std::string& key, int value) {
    if(!keyExists(key))
        setInt(key, value);
}

bool ConfigFile::keyExists(const std::string& key) const {
    return values.find(key) != values.end();
}

void ConfigFile::saveConfig() {
    std::ofstream file(path);
    for(auto& value : values)
        file << value.first + ":" + value.second + "\n";
    file.close();
}

ConfigFile::~ConfigFile() {
    saveConfig();
}

