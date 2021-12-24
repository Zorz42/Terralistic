#include "serverLiquids.hpp"

bool cmpLiquidUpdates(LiquidUpdate& a, LiquidUpdate& b) {
    return a.time > b.time;
}

void ServerLiquids::onEvent(ServerConnectionWelcomeEvent &event) {
    sf::Packet packet;
    packet << WelcomePacketType::LIQUIDS;
    event.connection->send(packet);
    
    event.connection->send(toSerial());
}

void ServerLiquids::init() {
    networking->connection_welcome_event.addListener(this);
    liquid_change_event.addListener(this);
    blocks->block_change_event.addListener(this);
    liquid_update_queue = std::priority_queue<LiquidUpdate, std::vector<LiquidUpdate>, bool(*)(LiquidUpdate&, LiquidUpdate&)>(cmpLiquidUpdates);
    
    liquid_schedules = new bool[getWidth() * getHeight()];
    
    for(int x = 0; x < getWidth(); x++)
        for(int y = 0; y < getHeight(); y++)
            getLiquidSchedule(x, y) = false;
    
    for(int x = 1; x < getWidth() - 1; x++)
        for(int y = 1; y < getHeight() - 1; y++)
            updateLiquid(x, y);
}

void ServerLiquids::stop() {
    networking->connection_welcome_event.removeListener(this);
    liquid_change_event.removeListener(this);
    blocks->block_change_event.removeListener(this);
}

bool& ServerLiquids::getLiquidSchedule(int x, int y) {
    return liquid_schedules[y * getWidth() + x];
}

bool ServerLiquids::isLiquidScheduled(int x, int y) {
    return getLiquidSchedule(x, y);
}

void ServerLiquids::update(float frame_length) {
    int count = 0;
    while(count < 300 && !liquid_update_queue.empty() && liquid_update_queue.top().time < gfx::getTicks()) {
        LiquidUpdate curr = liquid_update_queue.top();
        liquid_update_queue.pop();
        updateLiquid(curr.x, curr.y);
        getLiquidSchedule(curr.x, curr.y) = false;
        count++;
    }
}

void ServerLiquids::scheduleLiquidUpdate(int x, int y) {
    if(!isLiquidScheduled(x, y) && getLiquidType(x, y) != &empty) {
        getLiquidSchedule(x, y) = true;
        LiquidUpdate liquid_update;
        liquid_update.x = x;
        liquid_update.y = y;
        liquid_update.time = gfx::getTicks() + getLiquidType(x, y)->flow_time;
        liquid_update_queue.push(liquid_update);
    }
}

void ServerLiquids::scheduleLiquidUpdateForNeighbours(int x, int y) {
    scheduleLiquidUpdate(x, y);
    if(x > 0)
        scheduleLiquidUpdate(x - 1, y);
    if(y > 0)
        scheduleLiquidUpdate(x, y - 1);
    if(x < getWidth() - 1)
        scheduleLiquidUpdate(x + 1, y);
    if(y < getHeight() - 1)
        scheduleLiquidUpdate(x, y + 1);
}

void ServerLiquids::onEvent(LiquidChangeEvent& event) {
    scheduleLiquidUpdateForNeighbours(event.x, event.y);
    
    sf::Packet packet;
    packet << ServerPacketType::LIQUID << event.x << event.y << getLiquidType(event.x, event.y)->id << getLiquidLevel(event.x, event.y);
    networking->sendToEveryone(packet);
}

void ServerLiquids::onEvent(BlockChangeEvent& event) {
    scheduleLiquidUpdateForNeighbours(event.x, event.y);
}
