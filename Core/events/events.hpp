//
//  events.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 20/02/2021.
//

#ifndef events_hpp
#define events_hpp

#define REGISTER_EVENT(event_name) inline events::eventType event_name = events::generateUniqueEvent(); struct event_name ## _data

#include <set>

namespace events {

typedef void(*listenerFunction)(void* data_);

typedef unsigned char eventType;
void callEvent(eventType type, void* data);

struct eventListener {
    eventListener();
    virtual ~eventListener();
    std::set<eventType> events_listening_to;
    virtual void onEvent(eventType type, void* data) = 0;
};

eventType generateUniqueEvent();

}

#endif /* events_hpp */
