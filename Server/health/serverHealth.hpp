#pragma once
#include "serverNetworking.hpp"
#include "entities.hpp"
#include "serverPlayers.hpp"
#include "serverEntities.hpp"

class ServerHealth : public ServerModule, EventListener<EntityAbsoluteVelocityChangeEvent>, EventListener<ServerPacketEvent>{
    void init() override;
    void stop() override;
    void healthChange(ServerPlayer* curr_player);

    ServerPlayers* players;
    ServerEntities* server_entities;
    void onEvent(EntityAbsoluteVelocityChangeEvent &event) override;
    void onEvent(ServerPacketEvent &event) override;

public:
    ServerHealth(ServerPlayers* players, ServerEntities* server_entities): players(players), server_entities(server_entities){}
};
