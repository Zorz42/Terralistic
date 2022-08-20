#include "print.hpp"
#include <iomanip>
#include <iostream>
#include <sstream>

void Print::printText(MessageType type, const std::string& text) {
    auto t = std::time(nullptr);
    auto tm = *localtime(&t);
    std::stringstream timestamped_text;
    timestamped_text << std::put_time(&tm, "[%d.%m.%Y %H:%M:%S] ") << text;
    std::cout << timestamped_text.str() << std::endl;
    PrintEvent event(type, timestamped_text.str());
    print_event.call(event);
}

void Print::warning(const std::string &text) {
    printText(MessageType::WARNING, "[WARNING] " + text);
}

void Print::error(const std::string& text) {
    printText(MessageType::ERROR, "[ERROR] " + text);
}

void Print::info(const std::string& text) {
    printText(MessageType::INFO, text);
}
