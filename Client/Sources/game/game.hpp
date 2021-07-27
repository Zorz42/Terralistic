#ifndef game_hpp
#define game_hpp

#include "graphics.hpp"
#include "clientPlayers.hpp"
#include "clientItems.hpp"

void startPrivateWorld(const std::string& world_name);

class game : public gfx::Scene, EventListener<ClientPacketEvent> {
    void onEvent(ClientPacketEvent& event) override;
    void init() override;
    void update() override;
    void render() override;
    void stop() override;
    const std::string ip_address;
    const unsigned short port;
    std::string username;
    networkingManager networking_manager;
    ResourcePack resource_pack;
    ClientBlocks *blocks;
    ClientPlayers* player_handler;
    ClientItems* items;

public:
    game(std::string username, std::string ip_address, unsigned short port=33770) : ip_address(std::move(ip_address)), port(port), username(username) {}
};

#endif
