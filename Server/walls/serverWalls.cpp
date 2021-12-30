#include "serverWalls.hpp"

void ServerWalls::init() {
    world_saver->world_save_event.addListener(this);
    world_saver->world_load_event.addListener(this);
    networking->connection_welcome_event.addListener(this);
    wall_change_event.addListener(this);
}

void ServerWalls::stop() {
    world_saver->world_save_event.removeListener(this);
    world_saver->world_load_event.removeListener(this);
    networking->connection_welcome_event.removeListener(this);
    wall_change_event.removeListener(this);
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
    event.connection->send(packet);
    
    event.connection->send(toSerial());
}

void ServerWalls::onEvent(WallChangeEvent& event) {
    sf::Packet packet;
    packet << ServerPacketType::WALL << event.x << event.y << getWallType(event.x, event.y)->id;
    networking->sendToEveryone(packet);
    
}
