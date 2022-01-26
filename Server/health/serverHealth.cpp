#include "serverHealth.hpp"

void Health::init() {
    health_entities->entity_velocity_change_event.addListener(this);
}

void Health::onEvent(EntityVelocityChangeEvent &event) {
    if(event.entity->type == EntityType::PLAYER){
        ServerPlayer* player_to_change = players->getPlayerById(event.entity->id);
        if(std::abs(player_to_change->getVelocityY()) - event.oldVelY > 10){
            //change health accordingly
        }else if(std::abs(player_to_change->getVelocityX()) - event.oldVelX > 10){
            //change health accordingly
        }
    }
}


void Health::stop() {
    health_entities->entity_velocity_change_event.removeListener(this);
}

