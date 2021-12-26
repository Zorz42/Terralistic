#include "clientWalls.hpp"

void ClientWalls::onEvent(WelcomePacketEvent& event) {
    if(event.packet_type == WelcomePacketType::WALLS)
        fromSerial(networking->getData());
}

void ClientWalls::init() {
    networking->welcome_packet_event.addListener(this);
}

void ClientWalls::stop() {
    networking->welcome_packet_event.removeListener(this);
}

void ClientWalls::postInit() {
    
}

void ClientWalls::loadTextures() {
    
}

void ClientWalls::render() {
    
}

void ClientWalls::update(float frame_length) {
    
}
