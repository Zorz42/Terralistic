#pragma once
#include "camera.hpp"
#include "resourcePack.hpp"

class Background : public ClientModule {
    void loadTextures() override;
    void render() override;
    
    gfx::Texture background;
    
    Camera* camera;
    ResourcePack* resource_pack;
public:
    Background(Camera* camera, ResourcePack* resource_pack) : ClientModule("Background"), camera(camera), resource_pack(resource_pack) {}
};
