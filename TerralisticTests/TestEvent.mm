#include <XCTest/XCTest.h>
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

@interface TestEvent : XCTestCase

@end

@implementation TestEvent

- (void)testEvent {
    EventSender<DummyEvent> dummy_event;
    DummyEventListener dummy_event_listener;
    dummy_event.addListener(&dummy_event_listener);
    
    DummyEvent event(100);
    dummy_event.call(event);
    XCTAssertEqual(last_value, 100);
    
    dummy_event.removeListener(&dummy_event_listener);
    DummyEvent event2(200);
    dummy_event.call(event2);
    XCTAssertEqual(last_value, 100);
}

@end
