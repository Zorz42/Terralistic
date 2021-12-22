#pragma once
#include "clientModule.hpp"
#include "liquids.hpp"
#include "items.hpp"

class ResourcePack : public ClientModule {
    gfx::Texture hearts;
    std::vector<std::string> paths;
    
    void init() override;
public:
    const gfx::Texture& getHeartTexture();
    std::string getFile(const std::string& file_name);
};
