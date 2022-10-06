#include "clientHealth.hpp"
#include "readOpa.hpp"

void ClientHealth::init() {
    networking->packet_event.addListener(this);
    players->player_creation_event.addListener(this);
    players->player_deletion_event.addListener(this);
}

void ClientHealth::loadTextures() {
    heart_texture.loadFromSurface(readOpa(resource_pack->getFile("/misc/hearts.opa")));
}

void ClientHealth::stop() {
    networking->packet_event.removeListener(this);
    players->player_creation_event.removeListener(this);
    players->player_deletion_event.removeListener(this);
}

void ClientHealth::update(float frame_length) {
    enabled = players->getMainPlayer() != nullptr;
}

void ClientHealth::render() {
    while(!damage_texts.empty() && damage_texts[0]->hasDespawned()) {
        delete damage_texts[0];
        damage_texts.erase(damage_texts.begin());
    }
    
    for(DamageText* text : damage_texts)
        text->render(camera);
    
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

void ClientHealth::onEvent(PlayerCreationEvent &event) {
    event.player->health_change_event.addListener(this);
}

void ClientHealth::onEvent(PlayerDeletionEvent &event) {
    event.player->health_change_event.removeListener(this);
}

void ClientHealth::onEvent(PlayerHealthChangeEvent &event) {
    damage_texts.push_back(new DamageText(event.prev_health - event.player->getHealth(), event.player->getX(), event.player->getY()));
}

bool DamageText::hasDespawned() {
    return timer.getTimeElapsed() > 1000;
}

void DamageText::render(Camera* camera) {
    if(!initialized) {
        initialized = true;
        gfx::Color color = damage > 0 ? gfx::Color{200, 0, 0, 255} : gfx::Color{0, 200, 0, 255};
        texture.loadFromSurface(gfx::textToSurface(std::to_string(std::abs(damage)), color));
    }
    
    offset /= 6.0 / 5.0;
    
    float render_opacity = 1;
    if(timer.getTimeElapsed() > 700)
        render_opacity = (1000 - timer.getTimeElapsed()) / 300.f;
    
    int render_x = x - camera->getX() + gfx::getWindowWidth() / 2 + PLAYER_WIDTH - texture.getTextureWidth() * 1.5f / 2;
    int render_y = y - camera->getY() + gfx::getWindowHeight() / 2 - texture.getTextureHeight() * 1.5f - 3;
    texture.render(1.5f, render_x, render_y + offset, {0, 0, texture.getTextureWidth(), texture.getTextureHeight()}, false, {255, 255, 255, (unsigned char)(255 * render_opacity)});
}
