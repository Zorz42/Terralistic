//
//  print.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 11/01/2021.
//

#define FILENAME print
#define NAMESPACE print
#include "core.hpp"

#include "print.hpp"
#include <iomanip>
#include <ctime>

void print_(std::string text) {
    auto t = std::time(nullptr);
    auto tm = *std::localtime(&t);
    std::cout << std::put_time(&tm, "[%d.%m.%Y %H:%M:%S] ") << text << std::endl;
}

void print::info(std::string text) {
    print_(text);
}

void print::warning(std::string text) {
    print_("[warning] " + text);
}

void print::error(std::string text) {
    print_("[error] " + text);
}
