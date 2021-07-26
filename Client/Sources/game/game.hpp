//
//  game.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/07/2020.
//

#ifndef game_hpp
#define game_hpp

#include <utility>

#include "graphics.hpp"
#include "playerHandler.hpp"
#include "clientNetworking.hpp"
#include "inventoryHandler.hpp"
#include "resourcePack.hpp"

void startPrivateWorld(const std::string& world_name);

class game : public gfx::Scene {
public:
    const std::string ip_address;
    const unsigned short port;
    networkingManager networking_manager;
    ClientBlocks *world_map{};
    MainPlayer main_player;
    ResourcePack resource_pack;

    game(std::string username, std::string ip_address, unsigned short port=33770) : ip_address(std::move(ip_address)), port(port) { main_player.name = std::move(username); }
    void init() override;
    void update() override;
    void stop() override;
};

#endif /* game_hpp */
