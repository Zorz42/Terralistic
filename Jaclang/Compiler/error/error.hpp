#pragma once

#include <string>

class Error {
public:
    Error(const std::string& message) : message(message) {}
    std::string message;
};

void printError(Error error);
