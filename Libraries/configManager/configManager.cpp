//
//  configManager.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 28/05/2021.
//

#include "configManager.hpp"

configFile::configFile(std::string path) : path(path) {
    std::ifstream file(path);
    std::string line;
    while(std::getline(file, line)) {
        if(line.empty())
            continue;
        int separator_index = 0;
        while(line[separator_index] != ':')
            separator_index++;
        values[line.substr(0, separator_index)] = line.substr(separator_index + 1, line.size());
    }
}

std::string configFile::get(std::string key) {
    return values[key];
}

void configFile::set(std::string key, std::string value) {
    values[key] = value;
}

void configFile::setDefault(std::string key, std::string value) {
    if(values.find(key) == values.end())
        set(key, value);
}

void configFile::save() {
    std::ofstream file(path);
    for(std::map<std::string, std::string>::iterator iter = values.begin(); iter != values.end(); iter++)
        file << iter->first + ":" + iter->second + "\n";
    file.close();
}

configFile::~configFile() {
    save();
}
