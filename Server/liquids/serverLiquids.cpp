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
    networking_manager->connection_welcome_event.addListener(this);
}
