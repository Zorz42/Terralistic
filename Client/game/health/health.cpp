#include "health.hpp"

void Health::init() {
    /*manager->packet_event.addListener(this);
    manager->welcome_packet_event.addListener(this);*/
}

void Health::stop() {
    /*manager->packet_event.removeListener(this);
    manager->welcome_packet_event.removeListener(this);*/
}

void Health::render() {
    for(int i = 0; i * 5 < health; i++){
        const gfx::Texture& texture = resource_pack->getHeartTexture();
        texture.render(2, 10 + (i * 25) % 250, 10 + (i - i % 10), gfx::RectShape(0, 0 + (std::min(std::max(0, 5 * (i + 1) - health), 44)) * 11, 11, 11));
    }
}

void Health::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        default: break;
    }
}