#pragma once
#include "clientBlocks.hpp"
#include "walls.hpp"

class ClientWalls : public Walls, public ClientModule, EventListener<ClientPacketEvent>, EventListener<WallChangeEvent>, EventListener<WelcomePacketEvent> {
    class RenderWall {
    public:
        RenderWall() : variation(rand()), state(16) {}
        int variation:8, state:8;
    };

    class RenderWallChunk {
        gfx::RectArray wall_rects;
        int wall_count = 0;
        bool is_created = false;
    public:
        bool isCreated() const { return is_created; }
        void create();
        bool has_update = true;
        void update(ClientWalls* walls, int x, int y);
        void render(ClientWalls* walls, int x, int y);
    };
    
    RenderWall* render_walls = nullptr;
    RenderWallChunk* wall_chunks = nullptr;
    RenderWall* getRenderWall(int x, int y);
    RenderWallChunk* getRenderWallChunk(int x, int y);
    
    gfx::TextureAtlas walls_atlas;
    gfx::Texture breaking_texture;
    
    void onEvent(ClientPacketEvent& event) override;
    void onEvent(WallChangeEvent& event) override;
    void onEvent(WelcomePacketEvent& event) override;
    
    void init() override;
    void postInit() override;
    void loadTextures() override;
    void render() override;
    void updateParallel(float frame_length) override;
    void stop() override;
    
    bool updateOrientationSide(int x, int y, int side_x, int side_y);
    void updateOrientationDown(int x, int y);
    void updateOrientationUp(int x, int y);
    void updateOrientationLeft(int x, int y);
    void updateOrientationRight(int x, int y);
    
    void scheduleWallUpdate(int x, int y);
    
    DebugMenu* debug_menu;
    gfx::Timer line_refresh_timer;
    int fps_count = 0;
    float render_time_sum = 0;
    DebugLine render_time_line;
    
    ResourcePack* resource_pack;
    ClientNetworking* networking;
    Camera* camera;
    ClientBlocks* blocks;

public:
    ClientWalls(DebugMenu* debug_menu, ClientBlocks* blocks, ResourcePack* resource_pack, ClientNetworking* networking, Camera* camera) : ClientModule("ClientWalls"), debug_menu(debug_menu), Walls(blocks), blocks(blocks), resource_pack(resource_pack), networking(networking), camera(camera) {}
    
    const gfx::Texture& getWallsAtlasTexture();
    gfx::RectShape getWallRectInAtlas(WallType* type);
    
    void updateState(int x, int y);
    void setState(int x, int y, int state);
    int getState(int x, int y);
};
