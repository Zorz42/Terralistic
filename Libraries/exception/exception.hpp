#pragma once
#include <stdexcept>
#include <string>

class Exception : std::exception {
public:
    Exception(const std::string& message) : message(message) {}
    const std::string message;
    const char* what() const throw();
};

class ValueException : Exception {
public:
    ValueException(const std::string& message) : Exception(message) {}
};
