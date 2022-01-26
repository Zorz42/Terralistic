#include "serverHealth.hpp"

void ServerHealth::init() {
    health_entities->entity_velocity_change_event.addListener(this);
}

void ServerHealth::onEvent(EntityVelocityChangeEvent &event) {
    if(event.entity->type == EntityType::PLAYER){
        ServerPlayer* player_to_change = players->getPlayerById(event.entity->id);
        if(std::abs(player_to_change->getVelocityY()) - event.oldVelY > 10){
            //change health accordingly
        }else if(std::abs(player_to_change->getVelocityX()) - event.oldVelX > 10){
            //change health accordingly
        }
    }
}


void ServerHealth::stop() {
    health_entities->entity_velocity_change_event.removeListener(this);
}

