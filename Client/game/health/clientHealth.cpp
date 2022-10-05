#include "clientHealth.hpp"
#include "readOpa.hpp"

void ClientHealth::init() {
    networking->packet_event.addListener(this);
}

void ClientHealth::loadTextures() {
    heart_texture.loadFromSurface(readOpa(resource_pack->getFile("/misc/hearts.opa")));
}

void ClientHealth::stop() {
    networking->packet_event.removeListener(this);
}

void ClientHealth::update(float frame_length) {
    enabled = players->getMainPlayer() != nullptr;
}

void ClientHealth::render() {
    int offset = gfx::getWindowWidth() - (std::min(PLAYER_MAX_HEALTH / 4, 10) + 1) * 25 + 15;
    for(int i = PLAYER_MAX_HEALTH / 4 - 1; i >= 0; i--)
        heart_texture.render(2, offset + (i * 25) % 250,  (10 + (i - i % 10)), gfx::RectShape(0, 0 + (std::min(std::max(0, 4 * (i + 1) - players->getMainPlayer()->getHealth()), 4)) * 11, 11, 11));
}

void ClientHealth::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case ServerPacketType::HEALTH:
            int health, id;
            event.packet >> health >> id;
            players->getPlayerById(id)->setHealth(health);
            break;
        default: break;
    }
}
