#ifndef playerHandler_hpp
#define playerHandler_hpp

#include <string>
#include <utility>
#include "graphics.hpp"
#include "clientBlocks.hpp"
#include "resourcePack.hpp"
#include "clientEntities.hpp"
#include "player.hpp"
#include "liquids.hpp"

#define PLAYER_WIDTH 14
#define PLAYER_HEIGHT 24

class ClientPlayer : public Player {
public:
    ClientPlayer(const std::string& name, int x, int y, unsigned short id);
    bool flipped = false;
    unsigned char texture_frame = 0;
    gfx::Texture name_text;
    unsigned int started_moving = 0;
    bool has_jumped = false;
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
    
    ClientBlocks* blocks;
    Liquids* liquids;
    NetworkingManager* manager;
    ResourcePack* resource_pack;
    Entities* entities;
public:
    ClientPlayers(NetworkingManager* manager, ClientBlocks* blocks, Liquids* liquids, ResourcePack* resource_pack, Entities* entities, const std::string& username);
    
    void renderPlayers();
    
    const ClientPlayer* getMainPlayer() { return main_player; }
};

#endif
