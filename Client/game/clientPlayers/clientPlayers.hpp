#ifndef playerHandler_hpp
#define playerHandler_hpp

#include <string>
#include <utility>
#include "graphics.hpp"
#include "clientBlocks.hpp"
#include "resourcePack.hpp"

class ClientPlayer {
public:
    ClientPlayer(std::string name, int x, int y) : name(std::move(name)), x(x), y(y) {}
    int x, y;
    bool flipped = false;
    const std::string name;
};

class MainPlayer : public ClientPlayer {
public:
    explicit MainPlayer(std::string name) : ClientPlayer(std::move(name), 0, 0) {}
    short velocity_x = 0, velocity_y = 0;
};

class OtherPlayer : public ClientPlayer {
public:
    explicit OtherPlayer(std::string name, int x, int y, unsigned short id) : ClientPlayer(std::move(name), x, y), id(id) {}
    const unsigned short id;
    gfx::Image name_text;
};

class ClientPlayers : public gfx::GraphicalModule, EventListener<ClientPacketEvent> {
    bool key_up = false, jump = false, key_left = false, key_right = false;
    
    MainPlayer main_player;
    std::vector<OtherPlayer*> other_players;
    OtherPlayer* getPlayerById(unsigned short id);
    
    void render(ClientPlayer& player_to_draw);
    void render(OtherPlayer& player_to_draw);

    bool isPlayerColliding();
    bool isPlayerTouchingGround();

    bool received_spawn_coords = false;
    
    void onKeyUp(gfx::Key key) override;
    void onKeyDown(gfx::Key key) override;
    void init() override;
    void update() override;
    void onEvent(ClientPacketEvent& event) override;
    
    ClientBlocks* blocks;
    networkingManager* manager;
    ResourcePack* resource_pack;
public:
    ClientPlayers(networkingManager* manager, ClientBlocks* world_map, ResourcePack* resource_pack, std::string username) : manager(manager), blocks(world_map), resource_pack(resource_pack), main_player(std::move(username)) {}
    
    unsigned short getPlayerWidth();
    unsigned short getPlayerHeight();
    
    void renderPlayers();
    
    const MainPlayer& getMainPlayer() { return main_player; }
};

#endif
