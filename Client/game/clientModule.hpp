#pragma once

#include "events.hpp"

class GameErrorEvent {
public:
    GameErrorEvent(const std::string& message) : message(message) {}
    std::string message;
};

class ClientModule : public gfx::SceneModule {
public:
    EventSender<GameErrorEvent> game_error_event;
    virtual void postInit() {}
};
