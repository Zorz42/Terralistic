#pragma once
#include "clientBlocks.hpp"
#include "walls.hpp"

class ClientWalls : public Walls, public ClientModule, EventListener<WelcomePacketEvent> {
    class RenderWall {
    public:
        RenderWall() : variation(rand()), state(16) {}
        int variation:8, state:8;
    };

    class WallChunk {
        gfx::RectArray wall_rects;
        bool is_created = false;
    public:
        bool isCreated() { return is_created; }
        void create(ClientWalls* walls, int x, int y);
        void update(ClientWalls* walls, int x, int y);
        void render(ClientWalls* walls, int x, int y);
    };
    
    RenderWall* render_walls = nullptr;
    WallChunk* wall_chunks = nullptr;
    RenderWall* getRenderWall(int x, int y);
    WallChunk* getWallChunk(int x, int y);
    
    gfx::TextureAtlas walls_atlas;
    
    void onEvent(WelcomePacketEvent& event) override;
    
    void init() override;
    void postInit() override;
    void loadTextures() override;
    void render() override;
    void update(float frame_length) override;
    void stop() override;
    
    bool updateOrientationSide(ClientBlocks* blocks, int x, int y, int side_x, int side_y);
    void updateOrientationDown(ClientBlocks* blocks, int x, int y);
    void updateOrientationUp(ClientBlocks* blocks, int x, int y);
    void updateOrientationLeft(ClientBlocks* blocks, int x, int y);
    void updateOrientationRight(ClientBlocks* blocks, int x, int y);
    
    ResourcePack* resource_pack;
    ClientNetworking* networking;
    Camera* camera;

public:
    ClientWalls(Blocks* blocks, ResourcePack* resource_pack, ClientNetworking* networking, Camera* camera) : Walls(blocks), resource_pack(resource_pack), networking(networking), camera(camera) {}
    
    const gfx::Texture& getWallsAtlasTexture();
    gfx::RectShape getWallRectInAtlas(WallType* type);
    
    void updateState(int x, int y);
    void setState(int x, int y, int state);
    int getState(int x, int y);
};
