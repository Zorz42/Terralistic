#include "testing.hpp"
#include "events.hpp"

class DummyEvent {
public:
    DummyEvent(int value) : value(value) {}
    int value;
};

int last_value = -1;

class DummyEventListener : public EventListener<DummyEvent> {
    void onEvent(DummyEvent &event) override {
        last_value = event.value;
    }
};

TEST_CLASS(TestEvent)

TEST_CASE(testEvent) {
    EventSender<DummyEvent> dummy_event;
    DummyEventListener dummy_event_listener;
    dummy_event.addListener(&dummy_event_listener);
    
    DummyEvent event(100);
    dummy_event.call(event);
    ASSERT(last_value == 100);
    
    dummy_event.removeListener(&dummy_event_listener);
    DummyEvent event2(200);
    dummy_event.call(event2);
    ASSERT(last_value == 100);
}

END_TEST_CLASS(TestEvent)
