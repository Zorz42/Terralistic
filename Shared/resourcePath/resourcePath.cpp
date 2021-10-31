#include <set>
#include <utility>
#include "resourcePath.hpp"

std::string removeEndUntilChars(std::string string, const std::set<char>& chars) {
    while(!string.empty() && chars.count(string.back()) == 0)
        string.pop_back();
    if(!string.empty())
        string.pop_back();
    return string;
}

std::string returnEndUntilChars(std::string string, const std::set<char>& chars) {
    int iter = (int)string.size() - 1;
    while(chars.count(string[iter]) == 0 && iter != 0)
        iter--;
    return string.substr(iter + 1, string.size());
}

std::string getResourcePath(std::string executable_path) {
    std::string parent_directory = removeEndUntilChars(std::move(executable_path), {'/', '\\'});

    std::string parent_parent_directory = removeEndUntilChars(parent_directory, {'/', '\\'}) + "/";
    std::string parent_directory_name = returnEndUntilChars(parent_directory, {'/', '\\'});

    if(parent_directory.empty())
        parent_directory = ".";

    return parent_directory_name == "MacOS" ? parent_parent_directory + "Resources/" : parent_directory + "/Resources/";
}
