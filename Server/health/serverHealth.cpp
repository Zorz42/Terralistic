#include "serverHealth.hpp"

void ServerHealth::init() {
    server_entities->entity_absolute_velocity_change_event.addListener(this);
}

void ServerHealth::onEvent(EntityAbsoluteVelocityChangeEvent &event) {
    ServerPlayer* curr_player = players->getPlayerById(event.entity->id);
    int delta_vel_x = std::abs(curr_player->getVelocityX() - event.old_vel_x);
    int delta_vel_y = std::abs(curr_player->getVelocityY() - event.old_vel_y);
    if(delta_vel_x + delta_vel_y > 69) {//70 - 150
        int health_decrease = delta_vel_x + delta_vel_y - 69;
        curr_player->setPlayerHealth(curr_player->health - health_decrease);
        healthChange(curr_player);
    }
}

void ServerHealth::onEvent(ServerPacketEvent &event) {
    if(event.packet_type == ClientPacketType::PLAYER_RESPAWN){
        event.player->setPlayerHealth(40);
        players->addPlayer(event.player->name);
    }
}

void ServerHealth::healthChange(ServerPlayer* curr_player) {
    if(curr_player->health <= 0){
        //empty inventory
        players->savePlayer(players->getPlayerById(curr_player->id));
        server_entities->removeEntity(server_entities->getEntityById(curr_player->id));
    }
}

void ServerHealth::stop() {
    server_entities->entity_absolute_velocity_change_event.removeListener(this);
}

