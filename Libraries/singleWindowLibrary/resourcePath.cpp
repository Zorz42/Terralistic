//
//  resourcePath.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 24/06/2020.
//

#include "singleWindowLibrary.hpp"

void swl_private::setResourcePath(std::string executable_path) {
    while(executable_path.at(executable_path.size()-1) != '/' && executable_path.at(executable_path.size()-1) != '\\')
        executable_path.pop_back();
    executable_path.pop_back();
    std::string parent_directory;
    while(executable_path.at(executable_path.size()-1) != '/' && executable_path.at(executable_path.size()-1) != '\\') {
        parent_directory.insert(parent_directory.begin(), executable_path.at(executable_path.size()-1));
        executable_path.pop_back();
    }
    swl_private::resourcePath = parent_directory == "MacOS" ? executable_path + "Resources/" : executable_path + parent_directory + "/Resources/";
}
