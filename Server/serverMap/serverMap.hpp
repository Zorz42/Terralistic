 //
//  serverMap.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/04/2021.
//

#ifndef serverMap_hpp
#define serverMap_hpp

#include <utility>
#include <vector>
#include <string>
#include <chrono>
#include "serverNetworking.hpp"
#include "SimplexNoise.h"

#define BLOCK_WIDTH 16
#define MAX_LIGHT 100
#define UNBREAKABLE -1

class serverMap : serverPacketListener {

public:
    
private:

    void onPacket(packets::packet& packet, connection& conn) override;

    serverNetworkingManager* manager;

public:
    serverMap(serverNetworkingManager* manager, std::string resource_path) : manager(manager), serverPacketListener(manager), resource_path(std::move(resource_path)) { listening_to = {packets::STARTED_BREAKING, packets::STOPPED_BREAKING, packets::RIGHT_CLICK, packets::CHUNK, packets::VIEW_SIZE_CHANGE, packets::PLAYER_MOVEMENT, packets::PLAYER_JOIN, packets::DISCONNECT, packets::INVENTORY_SWAP, packets::HOTBAR_SELECTION, packets::CHAT}; }

    void saveWorld(const std::string& world_path);
    void loadWorld(const std::string& world_path);

    ~serverMap();
};

#endif /* serverMap_hpp */
