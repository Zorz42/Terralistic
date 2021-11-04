#include "serverLiquids.hpp"

void ServerLiquids::onEvent(ServerConnectionWelcomeEvent &event) {
    sf::Packet packet;
    packet << WelcomePacketType::LIQUIDS;
    event.connection->send(packet);
    
    std::vector<char> block_data;
    serialize(block_data);
    event.connection->send(block_data);
}

void ServerLiquids::init() {
    networking->connection_welcome_event.addListener(this);
    liquid_change_event.addListener(this);
}

void ServerLiquids::stop() {
    networking->connection_welcome_event.removeListener(this);
    liquid_change_event.removeListener(this);
}

void ServerLiquids::onEvent(LiquidChangeEvent& event) {
    sf::Packet packet;
    packet << ServerPacketType::LIQUID << event.x << event.y << getLiquidType(event.x, event.y)->id << getLiquidLevel(event.x, event.y);
    networking->sendToEveryone(packet);
}
