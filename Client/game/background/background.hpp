#pragma once
#include "camera.hpp"
#include "resourcePack.hpp"

class GameBackground : public ClientModule {
    void loadTextures() override;
    void render() override;
    
    gfx::Texture background;
    
    Camera* camera;
    ResourcePack* resource_pack;
public:
    GameBackground(Camera* camera, ResourcePack* resource_pack) : ClientModule("GameBackground"), camera(camera), resource_pack(resource_pack) {}
};
