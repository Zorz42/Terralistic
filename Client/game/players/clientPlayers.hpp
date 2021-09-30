#ifndef playerHandler_hpp
#define playerHandler_hpp

#include <string>
#include <utility>
#include "graphics.hpp"
#include "blockRenderer.hpp"
#include "resourcePack.hpp"
#include "clientEntities.hpp"
#include "movingType.hpp"

#define PLAYER_WIDTH 14
#define PLAYER_HEIGHT 24

class ClientPlayer : public Entity {
public:
    ClientPlayer(const std::string& name, int x, int y, unsigned short id);
    bool flipped = false;
    unsigned char texture_frame = 0;
    const std::string name;
    gfx::Texture name_text;
    MovingType moving_type = MovingType::STANDING;
    unsigned int started_moving = 0;
    bool has_jumped = false;
    unsigned short getWidth() override { return PLAYER_WIDTH * 2; }
    unsigned short getHeight() override { return PLAYER_HEIGHT * 2; }
    bool isColliding(Blocks* blocks) override;
};

class ClientPlayers : public gfx::SceneModule, EventListener<ClientPacketEvent> {
    bool walking_left = false, walking_right = false, sneaking_left = false, sneaking_right = false, running_left = false, running_right = false;
    
    void render(ClientPlayer& player_to_draw);

    std::string username;
    ClientPlayer* main_player = nullptr;
    ClientPlayer* getPlayerById(unsigned short id);
    
    void init() override;
    void update() override;
    void onEvent(ClientPacketEvent& event) override;
    
    BlockRenderer* client_blocks;
    Blocks* blocks;
    Liquids* liquids;
    NetworkingManager* manager;
    ResourcePack* resource_pack;
    Entities* entities;
public:
    ClientPlayers(NetworkingManager* manager, Blocks* blocks, Liquids* liquids, BlockRenderer* client_blocks, ResourcePack* resource_pack, Entities* entities, const std::string& username);
    
    void renderPlayers();
    
    const ClientPlayer* getMainPlayer() { return main_player; }
};

#endif
