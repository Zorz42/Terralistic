#include "clientHealth.hpp"


void ClientHealth::init() {
    networking->packet_event.addListener(this);
    networking->welcome_packet_event.addListener(this);
}

void ClientHealth::loadTextures() {
    heart_texture.loadFromFile(resource_pack->getFile("/misc/hearts.png"));
}

void ClientHealth::stop() {
    networking->packet_event.removeListener(this);
    networking->welcome_packet_event.removeListener(this);
}


void ClientHealth::render() {
    int offset = gfx::getWindowWidth() - (std::min(max_health / 4, 10) + 1) * 25 + 15;
    for(int i = max_health / 4 - 1; i >= 0; i--)
        heart_texture.render(2, offset + (i * 25) % 250,  (10 + (i - i % 10)), gfx::RectShape(0, 0 + (std::min(std::max(0, 4 * (i + 1) - health), 4)) * 11, 11, 11));
}

void ClientHealth::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case ServerPacketType::HEALTH:
            event.packet >> health;
            break;
        default: break;
    }
}

void ClientHealth::onEvent(WelcomePacketEvent &event) {
    switch (event.packet_type) {
        case WelcomePacketType::HEALTH:
            event.packet >> health;
            break;
        default: break;
    }
}

