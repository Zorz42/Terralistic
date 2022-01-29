#include "serverWalls.hpp"

void ServerWalls::init() {
    networking->connection_welcome_event.addListener(this);
    wall_change_event.addListener(this);
    wall_started_breaking_event.addListener(this);
    wall_stopped_breaking_event.addListener(this);
    world_saver->world_load_event.addListener(this);
    world_saver->world_save_event.addListener(this);
}

void ServerWalls::update(float frame_length) {
    updateBreakingWalls(frame_length);
}

void ServerWalls::stop() {
    networking->connection_welcome_event.removeListener(this);
    wall_change_event.removeListener(this);
    wall_started_breaking_event.removeListener(this);
    wall_stopped_breaking_event.removeListener(this);
    world_saver->world_load_event.removeListener(this);
    world_saver->world_save_event.removeListener(this);
}

void ServerWalls::onEvent(WorldSaveEvent &event) {
    world_saver->setSectionData("walls", toSerial());
}

void ServerWalls::onEvent(WorldLoadEvent &event) {
    fromSerial(world_saver->getSectionData("walls"));
}

void ServerWalls::onEvent(ServerConnectionWelcomeEvent& event) {
    sf::Packet packet;
    packet << WelcomePacketType::WALLS;
    event.connection->sendDirectly(packet);
    
    event.connection->send(toSerial());
}

void ServerWalls::onEvent(WallChangeEvent& event) {
    sf::Packet packet;
    packet << ServerPacketType::WALL << event.x << event.y << getWallType(event.x, event.y)->id;
    networking->sendToEveryone(packet);
}

void ServerWalls::onEvent(WallStartedBreakingEvent& event) {
    sf::Packet packet;
    packet << ServerPacketType::WALL_STARTED_BREAKING << event.x << event.y;
    networking->sendToEveryone(packet);
}

void ServerWalls::onEvent(WallStoppedBreakingEvent& event) {
    sf::Packet packet;
    packet << ServerPacketType::WALL_STOPPED_BREAKING << event.x << event.y;
    networking->sendToEveryone(packet);
}
