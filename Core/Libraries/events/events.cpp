//
//  events.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 20/02/2021.
//

#include "core.hpp"

static std::vector<events::eventListener*> listeners;

events::eventType events::generateUniqueEvent() {
    static eventType type = 0;
    return type++;
}

events::eventListener::eventListener() {
    listeners.push_back(this);
}

events::eventListener::~eventListener() {
    for(int i = 0; i < listeners.size(); i++)
        if(listeners[i] == this) {
            listeners.erase(listeners.begin() + i);
            break;
        }
            
}

void events::callEvent(eventType type, void *data) {
    for(eventListener* listener : listeners)
        if(listener->events_listening_to.find(type) != listener->events_listening_to.end())
            listener->onEvent(type, data);
}
