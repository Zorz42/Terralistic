#include "print.hpp"
#include <iomanip>
#include <iostream>

static void printText(const std::string& text) {
    auto t = std::time(nullptr);
    auto tm = *localtime(&t);
    std::cout << std::put_time(&tm, "[%d.%m.%Y %H:%M:%S] ") << text << std::endl;
}

void print::info(const std::string& text) {
    printText(text);
}

void print::warning(const std::string& text) {
    printText("[warning] " + text);
}

void print::error(const std::string& text) {
    printText("[error] " + text);
}
