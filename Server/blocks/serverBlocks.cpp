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
    block_break_stage_change_event.addListener(this);
}

void ServerBlocks::onEvent(BlockChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::BLOCK << event.x << event.y << (unsigned char)getBlockType(event.x, event.y);
    networking->sendToEveryone(packet);
}

void ServerBlocks::onEvent(BlockBreakStageChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::BLOCK_PROGRESS << event.x << event.y << getBreakProgress(event.x, event.y);
    networking->sendToEveryone(packet);
}
