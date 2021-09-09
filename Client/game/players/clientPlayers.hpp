#ifndef playerHandler_hpp
#define playerHandler_hpp

#include <string>
#include <utility>
#include "graphics.hpp"
#include "clientBlocks.hpp"
#include "resourcePack.hpp"
#include "clientEntity.hpp"

enum class MovingType {STANDING, WALKING, SNEAKING, SNEAK_WALKING, RUNNING};

#define PLAYER_WIDTH 14
#define PLAYER_HEIGHT 24

class ClientPlayer : public ClientEntity {
public:
    ClientPlayer(const std::string& name, int x, int y, unsigned short id);
    bool flipped = false;
    unsigned char texture_frame = 0;
    const std::string name;
    gfx::Image name_text;
    unsigned short getWidth() override { return PLAYER_WIDTH * 2; }
    unsigned short getHeight() override { return PLAYER_HEIGHT * 2; }
};

class MainPlayer : public ClientPlayer {
public:
    explicit MainPlayer(const std::string& name) : ClientPlayer(name, 0, 0, 0) {}
    unsigned int started_moving = 0;
    bool has_jumped = false;
    MovingType moving_type = MovingType::STANDING;
};

class ClientPlayers : public gfx::GraphicalModule, EventListener<ClientPacketEvent> {
    bool walking_left = false, walking_right = false, sneaking_left = false, sneaking_right = false, running_left = false, running_right = false;
    
    void render(ClientPlayer& player_to_draw);

    MainPlayer main_player;
    std::vector<ClientPlayer*> other_players;
    ClientPlayer* getPlayerById(unsigned short id);
    bool isPlayerColliding();
    bool isPlayerTouchingGround();
    
    void init() override;
    void update() override;
    void onEvent(ClientPacketEvent& event) override;
    
    ClientBlocks* blocks;
    NetworkingManager* manager;
    ResourcePack* resource_pack;
    ClientEntities* entities;
public:
    ClientPlayers(NetworkingManager* manager, ClientBlocks* world_map, ResourcePack* resource_pack, ClientEntities* entities, std::string username);
    
    void renderPlayers();
    
    const MainPlayer& getMainPlayer() { return main_player; }
    void setMainPlayerPosition(int x, int y);
};

#endif
