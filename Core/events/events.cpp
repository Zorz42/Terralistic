//
//  events.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 20/02/2021.
//

#include "core.hpp"

struct eventListener {
    events::listenerFunction func;
    events::eventType type;
};

std::vector<eventListener>& getEventListeners() {
    static std::vector<eventListener> listeners;
    return listeners;
}

events::eventType events::generateUniqueEvent() {
    static eventType type = 0;
    return type++;
}

events::registerEventListener::registerEventListener(eventType type, listenerFunction func) {
    getEventListeners().push_back({func, type});
}

void events::callEvent(eventType type, void *data) {
    for(eventListener& listener : getEventListeners())
        if(listener.type == type)
            listener.func(data);
}
