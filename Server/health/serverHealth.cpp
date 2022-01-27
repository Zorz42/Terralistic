#include "serverHealth.hpp"

void ServerHealth::init() {
    health_entities->entity_absolute_velocity_change_event.addListener(this);
}

void ServerHealth::onEvent(EntityAbsoluteVelocityChangeEvent &event) {
    int delta_vel_x = std::abs(event.entity->getVelocityX() - event.old_vel_x);
    int delta_vel_y = std::abs(event.entity->getVelocityY() - event.old_vel_y);
    if(delta_vel_x + delta_vel_y > 99){
        int i = 1;
    }
}
//some function to change health

void ServerHealth::stop() {
    health_entities->entity_absolute_velocity_change_event.removeListener(this);
}

