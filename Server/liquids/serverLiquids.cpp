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

void ServerLiquids::onEvent(LiquidChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::LIQUID << event.x << event.y << (unsigned char)getLiquidType(event.x, event.y) << getLiquidLevel(event.x, event.y);
    networking->sendToEveryone(packet);
}
