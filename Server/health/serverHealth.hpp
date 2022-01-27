#pragma once
#include "serverNetworking.hpp"
#include "entities.hpp"
#include "serverPlayers.hpp"

class ServerHealth : public ServerModule, EventListener<EntityAbsoluteVelocityChangeEvent>{
    void init() override;
    void stop() override;

    Entities* health_entities;
    ServerPlayers* players;
    void onEvent(EntityAbsoluteVelocityChangeEvent &event) override;

public:
    ServerHealth(Entities* health_entities, ServerPlayers* players): health_entities(health_entities), players(players){}
};
