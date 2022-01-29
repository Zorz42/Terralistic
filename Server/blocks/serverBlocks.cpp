#include "serverBlocks.hpp"

void ServerBlocks::onEvent(ServerConnectionWelcomeEvent& event) {
    sf::Packet packet;
    packet << WelcomePacketType::BLOCKS;
    event.connection->sendDirectly(packet);
    
    event.connection->send(toSerial());
}

void ServerBlocks::init() {    
    networking->connection_welcome_event.addListener(this);
    block_change_event.addListener(this);
    block_started_breaking_event.addListener(this);
    block_stopped_breaking_event.addListener(this);
    world_saver->world_load_event.addListener(this);
    world_saver->world_save_event.addListener(this);
}

void ServerBlocks::update(float frame_length) {
    updateBreakingBlocks(frame_length);
}

void ServerBlocks::stop() {
    networking->connection_welcome_event.removeListener(this);
    block_change_event.removeListener(this);
    block_started_breaking_event.removeListener(this);
    block_stopped_breaking_event.removeListener(this);
    world_saver->world_load_event.removeListener(this);
    world_saver->world_save_event.removeListener(this);
}

void ServerBlocks::onEvent(BlockChangeEvent& event) {
    sf::Packet packet;
    packet << ServerPacketType::BLOCK << event.x << event.y << getBlockType(event.x, event.y)->id;
    networking->sendToEveryone(packet);
}

void ServerBlocks::onEvent(BlockStartedBreakingEvent& event) {
    sf::Packet packet;
    packet << ServerPacketType::BLOCK_STARTED_BREAKING << event.x << event.y;
    networking->sendToEveryone(packet);
}

void ServerBlocks::onEvent(BlockStoppedBreakingEvent& event) {
    sf::Packet packet;
    packet << ServerPacketType::BLOCK_STOPPED_BREAKING << event.x << event.y;
    networking->sendToEveryone(packet);
}

void ServerBlocks::onEvent(WorldSaveEvent &event) {
    world_saver->setSectionData("blocks", toSerial());
}

void ServerBlocks::onEvent(WorldLoadEvent &event) {
    fromSerial(world_saver->getSectionData("blocks"));
}
