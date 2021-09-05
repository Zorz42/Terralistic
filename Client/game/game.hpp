#ifndef game_hpp
#define game_hpp

#include "graphics.hpp"
#include "clientPlayers.hpp"
#include "clientItems.hpp"
#include "menuBack.hpp"
#include "menuBack.hpp"
#include "entity.hpp"

void startPrivateWorld(const std::string& world_name, BackgroundRect* menu_back, bool structure_world);

class game : public gfx::Scene, EventListener<ClientPacketEvent>, public BackgroundRect {
    void onEvent(ClientPacketEvent& event) override;
    void init() override;
    void update() override;
    void onKeyDown(gfx::Key key) override;
    void render() override;
    void stop() override;
    const std::string ip_address;
    const unsigned short port;
    std::string username;
    NetworkingManager networking_manager;
    ResourcePack resource_pack;
    EntityManager entity_manager;
    ClientBlocks blocks;
    ClientPlayers player_handler;
    ClientItems items;
    BackgroundRect* menu_back;
    bool handshake_done = false;
    void handshakeWithServer();
    
public:
    game(BackgroundRect* menu_back, std::string username, std::string ip_address, unsigned short port=33770);
    
    void renderBack() override;
    void setBackWidth(unsigned short width) override { }
    unsigned short getBackWidth() override { return 0; }
};

#endif
