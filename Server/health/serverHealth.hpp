#pragma once
#include "serverNetworking.hpp"
#include "entities.hpp"
#include "serverPlayers.hpp"

class ServerHealth : public ServerModule, EventListener<EntityVelocityChangeEvent>{
    int health = 80, max_health = 80;
    void init() override;
    void onEvent(EntityVelocityChangeEvent &event) override;
    void stop() override;

    //ServerNetworking* networking;//probably don't need lol
    Entities* health_entities;
    ServerPlayers* players;

public:
    //ClientHealth(ServerNetworking* networking) : networking(networking){}
    ServerHealth(Entities* health_entities, ServerPlayers* players): health_entities(health_entities), players(players){}
};