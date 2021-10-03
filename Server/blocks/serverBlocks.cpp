#include "serverBlocks.hpp"

void ServerBlocks::onEvent(ServerConnectionWelcomeEvent& event) {
    sf::Packet packet;
    packet << WelcomePacketType::BLOCKS;
    event.connection->send(packet);
    
    std::vector<char> block_data;
    serialize(block_data);
    event.connection->send(block_data);
}

void ServerBlocks::init() {
    networking_manager->connection_welcome_event.addListener(this);
}
