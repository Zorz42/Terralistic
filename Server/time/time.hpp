#pragma once
#include "serverNetworking.hpp"
#include "worldSaver.hpp"

#undef SECONDS_PER_DAY
#define SECONDS_PER_DAY (60 * 10)

class ServerTime : public ServerModule, EventListener<ServerNewConnectionEvent> {
    gfx::ModTimer timer;
    ServerNetworking* networking;
    
    void onEvent(ServerNewConnectionEvent& event) override;
    
    void init() override;
    void stop() override;
public:
    ServerTime(ServerNetworking* networking) : networking(networking), timer(SECONDS_PER_DAY * 1000) {}
    void setTime(int time);
};
