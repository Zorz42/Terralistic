#pragma once

#include <vector>
#include <iostream>
#include <algorithm>
#include "exception.hpp"

template<class EventInstance>
class EventListener {
public:
    virtual void onEvent(EventInstance& event) = 0;
};

template<class EventInstance>
class EventSender {
    std::vector<EventListener<EventInstance>*> listeners;
public:
    void addListener(EventListener<EventInstance>* listener){
        listeners.push_back(listener);
    }
    
    void removeListener(EventListener<EventInstance>* listener) {
        listeners.erase(std::find(listeners.begin(), listeners.end(), listener));
    }
    
    void call(EventInstance& event) {
        for(EventListener<EventInstance>* listener : listeners)
            listener->onEvent(event);
    }
    
    ~EventSender() {
        if(!listeners.empty())
            std::cout << "Warning: Event destructed with listeners!" << std::endl;
    }
};
