#pragma once
#include "clientNetworking.hpp"
#include "resourcePack.hpp"
#include "content.hpp"
#include "camera.hpp"

class ClientBlocks : public Blocks, public ClientModule, EventListener<ClientPacketEvent>, EventListener<BlockChangeEvent>, EventListener<WelcomePacketEvent> {
    class RenderBlock {
    public:
        RenderBlock() : variation(rand()), state(16) {}
        int variation:8, state:8;
    };

    class RenderBlockChunk {
        gfx::RectArray block_rects;
        int block_count = 0;
        bool is_created = false;
    public:
        bool isCreated() const { return is_created; }
        void create();
        bool has_update = true;
        void update(ClientBlocks* blocks, int x, int y);
        void render(ClientBlocks* blocks, int x, int y);
    };
    
    void onEvent(ClientPacketEvent& event) override;
    void onEvent(BlockChangeEvent& event) override;
    void onEvent(WelcomePacketEvent& event) override;
    
    RenderBlock* render_blocks = nullptr;
    RenderBlockChunk* block_chunks = nullptr;
    RenderBlock* getRenderBlock(int x, int y);
    
    gfx::TextureAtlas blocks_atlas;
    gfx::Texture breaking_texture;
    
    void init() override;
    void postInit() override;
    void loadTextures() override;
    void render() override;
    void updateParallel(float frame_length) override;
    void stop() override;
    
    void scheduleBlockUpdate(int x, int y);
    
    int view_begin_x = 0, view_begin_y = 0, view_end_x = 0, view_end_y = 0, extended_view_begin_x = 0, extended_view_begin_y = 0, extended_view_end_x = 0, extended_view_end_y = 0;
    
    ResourcePack* resource_pack;
    ClientNetworking* networking;
    Camera* camera;
    
public:
    ClientBlocks(ResourcePack* resource_pack, ClientNetworking* networking, Camera* camera) : resource_pack(resource_pack), networking(networking), camera(camera) {}
    
    const gfx::Texture& getBlocksAtlasTexture();
    gfx::RectShape getBlockRectInAtlas(BlockType* type);
    RenderBlockChunk* getRenderBlockChunk(int x, int y);
    
    int getBlocksViewBeginX() const;
    int getBlocksViewEndX() const;
    int getBlocksViewBeginY() const;
    int getBlocksViewEndY() const;
    
    int getBlocksExtendedViewBeginX() const;
    int getBlocksExtendedViewEndX() const;
    int getBlocksExtendedViewBeginY() const;
    int getBlocksExtendedViewEndY() const;
    
    void updateState(int x, int y);
    void setState(int x, int y, int state);
    int getState(int x, int y);
};
