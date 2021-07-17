#ifndef events_hpp
#define events_hpp

#include <vector>

template<class EventListeningTo>
class EventListener;

template<class ChildType>
class Event {
    template<class EventListeningTo> friend class EventListener;
    
    inline static std::vector<EventListener<ChildType>*> listeners;
public:
    void call() {
        for(EventListener<ChildType>* listener : listeners) {
            if(cancelled)
                break;
            listener->onEvent(*(ChildType*)this);
        }
    }
    
    bool cancelled = false;
};


template<class EventListeningTo>
class EventListener {
public:
    EventListener() {
        EventListeningTo::listeners.push_back(this);
    }
    
    virtual void onEvent(EventListeningTo& event) = 0;
};


#endif /* events_hpp */
