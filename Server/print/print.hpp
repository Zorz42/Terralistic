#pragma once
#include <string>
#include "events.hpp"

enum MessageType{INFO, WARNING, ERROR};

class PrintEvent {
public:
    PrintEvent(MessageType type, std::string message) : type(type), message(std::move(message)) {}
    MessageType type;
    std::string message;
};

struct Print{
    void printText(MessageType type, const std::string& text);
    void info(const std::string& text);
    void warning(const std::string& text);
    void error(const std::string& text);
    EventSender<PrintEvent> print_event;
};
