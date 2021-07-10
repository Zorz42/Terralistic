//
//  configManager.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 28/05/2021.
//

#include "configManager.hpp"

#include <utility>

configFile::configFile(const std::string& path) : path(path) {
    loadConfig();
}

void configFile::loadConfig() {
    std::ifstream file(path);
    std::string line;
    while(std::getline(file, line))
        if(!line.empty()) {
            unsigned long separator_index = line.find(':');
            values[line.substr(0, separator_index)] = line.substr(separator_index + 1, line.size());
        }
    file.close();
}

std::string configFile::getStr(const std::string& key) {
    return values[key];
}

int configFile::getInt(const std::string& key) {
    return std::stoi(getStr(key));
}

void configFile::setStr(const std::string& key, std::string value) {
    values[key] = value;
}
void configFile::setInt(const std::string& key, int value) {
    setStr(key, std::to_string(value));
}

void configFile::setDefaultStr(const std::string& key, std::string value) {
    if(!keyExists(key))
        setStr(key, value);
}

void configFile::setDefaultInt(const std::string& key, int value) {
    if(!keyExists(key))
        setInt(key, value);
}

bool configFile::keyExists(const std::string& key) {
    return values.find(key) != values.end();
}

void configFile::saveConfig() {
    std::ofstream file(path);
    for(auto& value : values)
        file << value.first + ":" + value.second + "\n";
    file.close();
    values.clear();
}

configFile::~configFile() {
    saveConfig();
}
