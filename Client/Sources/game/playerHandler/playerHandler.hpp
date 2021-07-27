#ifndef playerHandler_hpp
#define playerHandler_hpp

#include <string>
#include "graphics.hpp"
#include "clientBlocks.hpp"

class ClientPlayer {
public:
    int x, y;
    bool flipped;
    std::string name;
};

class MainPlayer : public ClientPlayer {
public:
    short velocity_x = 0, velocity_y = 0;
};

class OtherPlayer : public ClientPlayer {
public:
    unsigned short id{0};
    gfx::Image name_text;
};

class PlayerHandler : public gfx::GraphicalModule, EventListener<ClientPacketEvent> {
    bool key_up = false, jump = false, key_left = false, key_right = false;

    gfx::Image player_image;
    
    MainPlayer main_player;
    std::vector<OtherPlayer*> other_players;
    ClientBlocks* world_map;
    networkingManager* manager;
    OtherPlayer* getPlayerById(unsigned short id);
    
    void render(ClientPlayer& player_to_draw);
    void render(OtherPlayer& player_to_draw);
    
    void initRenderer();

    bool isPlayerColliding();
    bool touchingGround();

    bool received_spawn_coords = false;
    
    void onKeyUp(gfx::Key key) override;
    void onKeyDown(gfx::Key key) override;
    void init() override;
    void update() override;
    void render() override;
    void onEvent(ClientPacketEvent& event) override;
    
public:
    PlayerHandler(networkingManager* manager, ClientBlocks* world_map, std::string username) : manager(manager), world_map(world_map) { main_player.name = username; }
    
    unsigned short getPlayerWidth();
    unsigned short getPlayerHeight();
    
    inline const MainPlayer& getMainPlayer() { return main_player; }
};

#endif
