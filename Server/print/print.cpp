#include "print.hpp"
#include <iomanip>
#include <iostream>

void Print::printText(MessageType type, const std::string& text) {
    auto t = std::time(nullptr);
    auto tm = *localtime(&t);
    switch (type) {
        case 0: {
            std::cout << std::put_time(&tm, "[%d.%m.%Y %H:%M:%S] ") << text << std::endl;
            break;
        }
        case 1:{
            std::cout << std::put_time(&tm, "[%d.%m.%Y %H:%M:%S] ") << "[WARNING] " << text << std::endl;
            break;
        }
        case 2:{
            std::cout << std::put_time(&tm, "[%d.%m.%Y %H:%M:%S] ") << "[ERROR] " << text << std::endl;
            break;
        }
        default:
            break;
    }
    PrintEvent event(type, text);
    print_event.call(event);
}

void Print::info(const std::string& text) {
    printText(MessageType::INFO, text);
}

void Print::warning(const std::string& text) {
    printText(MessageType::WARNING, text);
}

void Print::error(const std::string& text) {
    printText(MessageType::ERROR, text);
}
