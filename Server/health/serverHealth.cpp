#include "serverHealth.hpp"

void ServerHealth::init() {
    health_entities->entity_velocity_change_event.addListener(this);
}

//some function to change health

void ServerHealth::stop() {
    health_entities->entity_velocity_change_event.removeListener(this);
}

