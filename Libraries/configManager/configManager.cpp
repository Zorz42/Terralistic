//
//  configManager.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 28/05/2021.
//

#include "configManager.hpp"

#include <utility>

configFile::configFile(const std::string& path) : path(path) {
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

std::string configFile::get(const std::string& key) {
    return values[key];
}

void configFile::set(const std::string& key, std::string value) {
    values[key] = std::move(value);
}

void configFile::setDefault(const std::string& key, std::string value) {
    if(values.find(key) == values.end())
        set(key, std::move(value));
}

void configFile::save() {
    std::ofstream file(path);
    for(auto & value : values)
        file << value.first + ":" + value.second + "\n";
    file.close();
}

configFile::~configFile() {
    save();
}
