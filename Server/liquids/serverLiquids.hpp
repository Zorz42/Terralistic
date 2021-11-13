#pragma once
#include "liquids.hpp"
#include "serverNetworking.hpp"

class ServerLiquids : public ServerModule, public Liquids, EventListener<ServerConnectionWelcomeEvent>, EventListener<LiquidChangeEvent> {
    ServerNetworking* networking;
    
    void onEvent(ServerConnectionWelcomeEvent& event) override;
    void onEvent(LiquidChangeEvent& event) override;
    
    void init() override;
    void stop() override;
public:
    ServerLiquids(Blocks* blocks, ServerNetworking* networking) : Liquids(blocks), networking(networking) {}
};
