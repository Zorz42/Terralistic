#include "health.hpp"

void Health::init() {
    networking->packet_event.addListener(this);
    networking->welcome_packet_event.addListener(this);
}

void Health::stop() {
    networking->packet_event.removeListener(this);
    networking->welcome_packet_event.removeListener(this);
}

void Health::render() {
    const gfx::Texture& texture = resource_pack->getHeartTexture();
    for(int i = max_health / 4 - 1; i >= 0; i--){
        texture.render(2, 10 + (i * 25) % 250,  (10 + (i - i % 10)), gfx::RectShape(0, 0 + (std::min(std::max(0, 4 * (i + 1) - health), 4)) * 11, 11, 11));
    }
}

void Health::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case ServerPacketType::HEALTH:
            event.packet >> health;
            break;
        default: break;
    }
}

void Health::onEvent(WelcomePacketEvent &event) {
    switch (event.packet_type) {
        case WelcomePacketType::HEALTH:
            event.packet >> health;
            break;
        default: break;
    }
}

