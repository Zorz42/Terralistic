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
    networking->connection_welcome_event.addListener(this);
    block_change_event.addListener(this);
    block_started_breaking_event.addListener(this);
    block_stopped_breaking_event.addListener(this);
}

void ServerBlocks::update(float frame_length) {
    updateBreakingBlocks(frame_length);
}

void ServerBlocks::stop() {
    networking->connection_welcome_event.removeListener(this);
    block_change_event.removeListener(this);
    block_started_breaking_event.removeListener(this);
    block_stopped_breaking_event.removeListener(this);
}

void ServerBlocks::onEvent(BlockChangeEvent& event) {
    sf::Packet packet;
    packet << ServerPacketType::BLOCK << event.x << event.y << (unsigned char)getBlockType(event.x, event.y);
    networking->sendToEveryone(packet);
}

void ServerBlocks::onEvent(BlockStartedBreakingEvent& event) {
    sf::Packet packet;
    packet << ServerPacketType::STARTED_BREAKING << event.x << event.y;
    networking->sendToEveryone(packet);
}

void ServerBlocks::onEvent(BlockStoppedBreakingEvent& event) {
    sf::Packet packet;
    packet << ServerPacketType::STOPPED_BREAKING << event.x << event.y;
    networking->sendToEveryone(packet);
}
