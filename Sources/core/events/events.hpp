//
//  events.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 20/02/2021.
//

#ifndef events_hpp
#define events_hpp

#define EVENT_LISTENER(type) events::registerEventListener UNIQUE_NAME(event_listener_registrator) (type, [](void* data_) { auto& data = *(JOIN(type, _data)*)data_;
#define EVENT_LISTENER_END });
#define REGISTER_EVENT(event_name) inline events::eventType event_name = events::generateUniqueEvent(); struct event_name ## _data

namespace events {

typedef void(*listenerFunction)(void* data_);

typedef unsigned char eventType;
void callEvent(eventType type, void* data);

eventType generateUniqueEvent();

struct registerEventListener {
    registerEventListener(eventType type, listenerFunction func);
};

}

#endif /* events_hpp */
