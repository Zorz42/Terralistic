#pragma once
#include "clientModule.hpp"
#include "liquids.hpp"
#include "items.hpp"

class ResourcePack : public ClientModule {
    std::vector<std::string> paths;
    
    void init() override;
public:
    ResourcePack() : ClientModule("ResourcePack") {}
    void loadPaths();
    std::string getFile(const std::string& file_name);
};
